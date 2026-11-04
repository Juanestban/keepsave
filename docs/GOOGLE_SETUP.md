# Google Drive setup

KeepSave talks to Google Drive with your own OAuth client. You set this up
once; it takes about 10 minutes.

## 1. Create a Google Cloud project

1. Go to <https://console.cloud.google.com/>.
2. Open the project picker, choose **New project**, name it `KeepSave` and
   click **Create**.
3. Make sure the new project is selected.

## 2. Enable the Drive API

1. Open **APIs & Services → Library**.
2. Search for **Google Drive API** and click **Enable**.

## 3. Configure the OAuth consent screen

1. Open **APIs & Services → OAuth consent screen** (shown as
   **Google Auth Platform → Branding / Audience** in the newer console).
2. User type: **External**.
3. App name `KeepSave`; use your email for the support and developer contacts.
4. **Scopes / Data access**: add
   `https://www.googleapis.com/auth/drive.file` (*See, edit, create and delete
   only the specific Google Drive files you use with this app*).
5. **Audience → Publishing status**: click **Publish app** to move it to
   **In production**.
   - In *Testing* mode, refresh tokens expire after 7 days and you would have
     to reconnect weekly.
   - `drive.file` is a non-sensitive scope, so publishing does not require
     Google's verification review. When you connect, Google shows a
     "Google hasn't verified this app" warning. Click **Advanced → Go to
     KeepSave** to continue.

## 4. Create the OAuth client

1. Open **APIs & Services → Credentials → Create credentials → OAuth client
   ID** (or **Google Auth Platform → Clients → Create client**).
2. Application type: **Desktop app**. Name: `KeepSave desktop`.
3. Click **Create** and copy the **Client ID** and **Client secret**.

For desktop apps, Google does not treat the client secret as confidential;
security comes from PKCE and the loopback redirect. Even so, keep it out of
public commits.

## 5. Give the credentials to KeepSave

KeepSave reads the credentials from environment variables at **build** time
and embeds them in the binary. Put them in a `.env` file at the repo root;
it is gitignored and loaded by the build script:

```env
KEEPSAVE_GOOGLE_CLIENT_ID=xxxxxxxx.apps.googleusercontent.com
KEEPSAVE_GOOGLE_CLIENT_SECRET=GOCSPX-xxxxxxxx
```

> The exact mechanism (build-time embedding vs. a field in Settings) will be
> confirmed in milestone 4. This file will be updated then.

## 6. Connect

In KeepSave, open **Settings → Google Drive → Connect**. A browser window
opens; sign in and approve. KeepSave creates a `KeepSave/` folder in your
Drive, with backups in `KeepSave/backups/`.

To disconnect, use **Settings → Google Drive → Disconnect**, which removes the
token from your OS credential store. You can also revoke access at
<https://myaccount.google.com/permissions>.
