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
welcome-coming-soon = Coming soon
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

## Unlocked vault (placeholder until the main screen lands)

vault-unlocked = Vault unlocked
vault-entries =
    { $count ->
        [one] 1 entry
       *[other] { $count } entries
    }
vault-coming-soon = The main screen arrives in the next milestone.
vault-lock = Lock
