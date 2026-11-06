# Google Drive setup

KeepSave talks to Google Drive with your own OAuth client. You set this up
once; it takes about 10 minutes.

KeepSave walks you through these same steps in the app: choose **Set up
Google Drive** (on the welcome screen's import, in the vault's sidebar, or in
**Settings → Google Drive**). Each step has a button that opens the right
Google Cloud page and chips that copy the exact values to type.

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

Paste the **Client ID** and **Client secret** in the last step of the guide
and click **Save and connect**. KeepSave stores them in the OS credential
store (Windows Credential Manager), never in `config.toml`. To replace or
remove them later, use **Settings → Google Drive → Google client**.

KeepSave never builds a client into the app, not even in your own builds:
anything embedded in an `.exe` can be read back, so every user sets up
their own client here.

## 6. Connect

The guide connects right after you save the client. Later, open
**Settings → Google Drive** (the gear icon) and click **Connect Drive**, or
unlock a vault and click **Connect Google Drive** at the bottom of the
sidebar. A browser tab opens; sign in and approve. Then click
**Upload to Drive** in the vault's sidebar to copy it into a `KeepSave/`
folder in your Drive.

On another PC, choose **Import from Google Drive** on the welcome screen.

To disconnect, click **Disconnect** in **Settings → Google Drive**, or
right-click the Drive row in the sidebar and choose **Disconnect Google
Drive**. The same Settings tab lists the linked vaults and unlinks them. KeepSave revokes the token and removes it from
your OS credential store. You can also revoke access at
<https://myaccount.google.com/permissions>.
