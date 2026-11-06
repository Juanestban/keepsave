app-name = KeepSave
app-tagline = Your passwords, encrypted and in sync.

language-es = Español
language-en = English

common-back = Back
reveal-show = Show password
reveal-hide = Hide password
error-generic = Something went wrong: { $details }

dialog-open-title = Open a KeePass vault
dialog-create-title = Save your new vault
dialog-filter = KeePass vault

## Welcome

welcome-create = Create a new vault
welcome-open = Open a .kdbx file
welcome-import-drive = Import from Google Drive
welcome-recent = Back to recent vaults

## Create

create-title = Create a vault
create-subtitle = Your vault is a standard KeePass file, encrypted with your master password.
create-name = Vault name
create-name-default = My vault
create-password = Master password
create-confirm = Confirm master password
create-warning = A forgotten master password can't be recovered. Keep it somewhere safe.
create-submit = Choose location and create
create-busy = Securing your vault…
create-error-name = Give your vault a name.
create-error-empty = Choose a master password.
create-error-mismatch = The passwords don't match.

strength-weak = Weak
strength-fair = Fair
strength-good = Good
strength-strong = Strong
strength-bits = ~{ $bits } bits

## Unlock

unlock-title = Unlock your vault
unlock-recent = Recent vaults
unlock-select = Select a vault to unlock.
unlock-password = Master password
unlock-submit = Unlock
unlock-busy = Unlocking…
unlock-forget = Remove from list
unlock-missing-file = File not found
unlock-open-other = Open another vault
unlock-create = Create a new vault
unlock-error-wrong-password = Wrong master password.
unlock-error-not-found = The vault file could not be found.

## Main screen

entries-count =
    { $count ->
        [one] 1 entry
       *[other] { $count } entries
    }
main-search = Search entries (Ctrl+F)
main-search-clear = Clear search
main-new-entry = New entry
main-lock = Lock (Ctrl+L)
main-groups = Groups
main-new-group = New group
main-recycle-bin = Recycle bin
main-results = Results for “{ $query }”
main-empty-group = No entries here yet.
main-empty-bin = The recycle bin is empty.
main-no-results = Nothing matches your search.
main-copy-username = Copy username (Ctrl+B)
main-copy-password = Copy password (Ctrl+C)

group-rename = Rename
group-delete = Move to recycle bin
bin-empty = Empty recycle bin

status-saved = Saved
status-saving = Saving…
status-failed = Not saved
status-retry = Click to try again. { $details }

toast-copied-username = Username copied
toast-copied-password = Password copied
toast-copied-url = URL copied
toast-copied-field = { $field } copied
toast-clears-in = clears in { $seconds } s
toast-copy-failed = Couldn't access the clipboard.
toast-moved-to-bin = Moved to the recycle bin
toast-restored = Restored
toast-version-restored = Version restored
toast-url-invalid = Only web links (http or https) can be opened.
toast-saving-before-lock = Saving before locking…
toast-lock-save-failed = Couldn't save, so the vault stays unlocked.

field-title = Title
field-username = Username
field-password = Password
field-url = URL
field-notes = Notes
field-tags = Tags
field-tags-hint = Separate tags with commas.
field-group = Group
field-custom = Custom fields
field-custom-empty = No custom fields. Add one for things like a PIN or recovery codes.
field-add = Add field
field-name = Field name
field-protect = Protect this value
field-unprotect = Store as plain text
field-remove = Remove field

detail-untitled = (untitled)
detail-edit = Edit (Enter)
detail-delete = Move to recycle bin (Delete)
detail-delete-forever = Delete permanently
detail-restore = Restore from the recycle bin
detail-close = Close (Esc)
detail-copy = Copy
detail-open-url = Open URL (Ctrl+U)
detail-modified = Modified { $date }
detail-history = History ({ $count })
detail-history-restore = Restore

edit-new-title = New entry
edit-title = Edit entry
edit-save = Save
edit-cancel = Cancel
edit-error-field-empty = Every custom field needs a name.
edit-error-field-duplicate = There are two custom fields named “{ $name }”.
edit-error-field-reserved = “{ $name }” is a reserved name. Pick another one.

gen-toggle = Generate
gen-hide = Hide generator
gen-chars = Characters
gen-words = Passphrase
gen-length = Length
gen-word-count = Words
gen-separator = Separator
gen-ambiguous = Avoid look-alikes (0 O 1 l I)
gen-regenerate = Generate another
gen-use = Use this password
gen-error-sets = Select at least one character set.

dialog-cancel = Cancel
dialog-new-group-title = New group
dialog-rename-group-title = Rename group
dialog-group-error-empty = Give the group a name.
dialog-create = Create
dialog-rename = Rename
dialog-delete-forever-title = Delete permanently?
dialog-delete-forever-body = “{ $title }” will be deleted for good. This can't be undone.
dialog-delete = Delete
dialog-empty-bin-title = Empty the recycle bin?
dialog-empty-bin-body = Everything in the recycle bin will be deleted for good. This can't be undone.
dialog-empty = Empty
dialog-discard-title = Discard your changes?
dialog-discard-body = Your changes to this entry haven't been saved.
dialog-discard = Discard
dialog-keep-editing = Keep editing
dialog-close-title = Close without saving?
dialog-close-body = The vault couldn't be saved. { $details }
dialog-close-anyway = Close anyway
dialog-stay = Stay

## Google Drive

drive-title = Import from Google Drive
drive-subtitle = Pick a vault from the KeepSave folder in your Drive.
drive-unavailable = Google Drive isn't set up yet.
drive-setup = Set up Google Drive
drive-setup-hint = KeepSave connects to Drive through a private Google client that you create once. A step-by-step guide walks you through it.
drive-connect = Connect Google Drive
drive-connect-short = Connect Drive
drive-connect-hint = A browser tab opens so you can sign in to Google. KeepSave can only see the files it creates.
drive-waiting = Waiting for the browser…
drive-cancel = Cancel
drive-cancel-tooltip = Click to cancel
drive-loading = Looking for vaults…
drive-picking = Choose where to save it…
drive-downloading = Downloading { $name }…
drive-empty = There are no vaults in your Drive's KeepSave folder yet.
drive-retry = Try again
drive-upload = Upload to Drive
drive-uploading = Uploading…
drive-linked = On Google Drive
drive-linked-tooltip = Linked to “{ $name }” on Drive. Click to sync now.
drive-unlink = Unlink this vault
drive-disconnect = Disconnect Google Drive
drive-error-denied = Access wasn't granted in the browser.
drive-error-timeout = Signing in took too long. Try again.
drive-error-reauthorize = Google Drive was disconnected. Connect it again.
drive-error-network = Couldn't reach Google. Check your connection.
drive-browser-success-title = KeepSave is connected
drive-browser-success-body = You can close this tab and go back to KeepSave.
drive-browser-denied-title = KeepSave wasn't connected
drive-browser-denied-body = Access wasn't granted. You can close this tab.
dialog-import-title = Save the vault on this PC
toast-drive-uploaded = Vault uploaded to Google Drive

## Sync

sync-now = Sync now
sync-running = Syncing…
sync-done = Synced at { $time }
sync-pending = Upload pending
sync-offline = Waiting for the network
sync-offline-tooltip = Your changes are saved on this PC. KeepSave retries every minute, or click to try now.
sync-failed = Sync failed
toast-drive-merged =
    { $count ->
        [one] 1 change from Google Drive
       *[other] { $count } changes from Google Drive
    }
drive-error-missing = This vault is no longer on Google Drive. Unlink it, then upload it again.
drive-error-password = The copy on Google Drive has a different master password, so it wasn't touched.

## Settings

settings-open = Settings
settings-title = Settings
settings-close = Close
settings-tab-general = General
settings-tab-security = Security
settings-tab-drive = Google Drive
settings-tab-language = Language
settings-tab-appearance = Appearance
settings-tab-about = About
settings-config-error = Your settings couldn't be saved: { $details }
settings-recent = Recent vaults
settings-recent-hint = Removing a vault from this list doesn't delete its file.
settings-recent-empty = No recent vaults.
settings-recent-remove = Remove from the list
settings-recent-clear = Clear the list
settings-never = Never
settings-minutes = { $count } min
settings-seconds = { $count } s
settings-lock-after = Lock after inactivity
settings-lock-after-hint = Without mouse or keyboard input for this long.
settings-lock-minimize = Lock when the window is minimized
settings-lock-sleep = Lock when the PC sleeps or the session locks
settings-lock-sleep-windows = Only available on Windows for now.
settings-clipboard-clear = Clear copied secrets after
settings-clipboard-clear-hint = Only if the clipboard still holds what KeepSave copied.
settings-clipboard-keep = Never
settings-drive-connection = Connection
settings-drive-connected = Connected to Google Drive
settings-drive-disconnected = Not connected
settings-drive-disconnect = Disconnect
settings-drive-links = Linked vaults
settings-drive-links-empty = No vault is linked yet. Open a vault and choose “Upload to Drive” in its sidebar.
settings-drive-unlink = Unlink this vault
settings-drive-setup-hint = Create your own Google client with the guide. It takes about 10 minutes.
settings-drive-client = Google client
settings-drive-client-user = Your own client
settings-drive-client-user-hint = Kept in the system credential store, never in the settings file.
settings-drive-client-change = Change
settings-drive-client-remove = Remove the client and disconnect
settings-language-system = System ({ $language })
settings-animated-background = Animated background
settings-animated-background-hint = When off, the background stays still and uses less power.

## About

about-version = Version { $version }
about-license = Free software under the MIT or Apache 2.0 license.
about-repository = Source code on GitHub
about-credits = Credits
about-credit-inter = Inter typeface by Rasmus Andersson, under the SIL Open Font License 1.1.
about-credit-phosphor = Phosphor icons, under the MIT License.
about-credit-eff = Passphrase wordlist by the Electronic Frontier Foundation, under CC BY 3.0.
about-eff-link = EFF wordlists

about-license-title = License
about-updates = Updates

## Updates

update-available = Update available
update-ready = Update ready to install
update-versions = KeepSave { $version } · you have { $current }
update-close = Hide until next start
update-install = Update
update-notes = What's new
update-downloading = Downloading… { $percent }%
update-restart = Restart to update
update-restart-hint = An open vault is saved and locked first.
update-retry = Retry
update-open-page = Release page
update-check-now = Check now
update-automatic = Check for updates automatically
update-automatic-hint = At startup and every 6 hours. Only asks GitHub for the latest version.
update-status-idle = Not checked yet.
update-status-checking = Checking for updates…
update-status-up-to-date = KeepSave is up to date.
update-status-available = Version { $version } is available.
update-status-downloading = Downloading the update…
update-error-network = Couldn't reach GitHub. Check your connection.
update-error-rate-limit = GitHub is limiting requests. Try again later.
update-error-checksum = The download is damaged. Try again.
update-error-missing = This release can't be installed from the app. Download it from the release page.
update-error-folder = KeepSave can't write to its folder. Download the update from the release page.


## Google Drive setup guide

gsetup-title = Set up Google Drive
gsetup-close = Close the guide
gsetup-time = About 10 minutes
gsetup-step-of = Step { $step } of { $total }
gsetup-start = Start
gsetup-next = Next
gsetup-back = Back
gsetup-save = Save and connect
gsetup-open-cloud = Open in Google Cloud
gsetup-full-guide = Read the full guide on GitHub
gsetup-copied = Copied
gsetup-chip-project = Project name
gsetup-chip-app = App name
gsetup-chip-scope = Scope
gsetup-chip-client = Name
gsetup-client-id = Client ID
gsetup-client-secret = Client secret
gsetup-client-id-invalid = A client ID ends in .apps.googleusercontent.com.

gsetup-intro-title = Your own private connection to Drive
gsetup-intro-body = KeepSave talks to Google Drive through a Google client that belongs to you, so no one else's app is involved. You create it once, for free, in Google Cloud.
gsetup-intro-1 = You need a Google account. Use the one whose Drive will hold your vaults.
gsetup-intro-2 = Keep this window open. Each step has a button that opens the right page in Google Cloud.
gsetup-intro-3 = Click a value chip to copy it, then paste it where the step says.
gsetup-intro-tip = Your vault is encrypted before it leaves this PC, and KeepSave can only see the files it creates in your Drive.

gsetup-project-title = Create a project
gsetup-project-body = A project holds the settings of your Google client.
gsetup-project-1 = Name the project KeepSave and click Create.
gsetup-project-2 = Wait for the notification that the project is ready.
gsetup-project-3 = Check that KeepSave is selected in the project picker at the top of the page.

gsetup-api-title = Turn on the Google Drive API
gsetup-api-body = This lets your project talk to Google Drive.
gsetup-api-1 = Check that the selected project at the top is KeepSave.
gsetup-api-2 = Click Enable and wait for the API page to load.

gsetup-branding-title = Describe the app
gsetup-branding-body = This is what Google shows when you sign in from KeepSave.
gsetup-branding-1 = Click Get started.
gsetup-branding-2 = App name: KeepSave. User support email: your email. Click Next.
gsetup-branding-3 = Audience: External. Click Next.
gsetup-branding-4 = Contact email: your email. Click Next, accept the policy and click Create.

gsetup-scope-title = Allow access to KeepSave's files
gsetup-scope-body = The permission KeepSave will ask for.
gsetup-scope-1 = Click Add or remove scopes.
gsetup-scope-2 = Under Manually add scopes, paste the scope and click Add to table.
gsetup-scope-3 = Click Update, then Save at the bottom of the page.
gsetup-scope-tip = This scope lets KeepSave see only the files it creates, never the rest of your Drive.

gsetup-publish-title = Publish the app
gsetup-publish-body = Move the app out of testing so the connection lasts.
gsetup-publish-1 = Under Publishing status, click Publish app.
gsetup-publish-2 = Confirm. The status changes to In production.
gsetup-publish-tip = In testing, Google disconnects KeepSave every 7 days. This scope doesn't need Google's review to publish.

gsetup-client-title = Create the client
gsetup-client-body = The client is the key pair KeepSave uses to sign you in.
gsetup-client-1 = Application type: Desktop app.
gsetup-client-2 = Name: KeepSave desktop. Click Create.
gsetup-client-3 = Copy the Client ID and the Client secret from the dialog that opens.
gsetup-client-tip = Download JSON in that dialog keeps a copy. Store it somewhere private.

gsetup-paste-title = Paste your client
gsetup-paste-body = KeepSave keeps it in the system credential store, never in its settings file.
gsetup-paste-tip = When you connect, Google warns that it hasn't verified the app. It's your own app: click Advanced, then Go to KeepSave.

## Password generator

generator-open = Password generator
generator-title = Password generator
gen-copy = Copy
