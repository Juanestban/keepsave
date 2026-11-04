app-name = KeepSave
app-tagline = Tus contraseñas, cifradas y sincronizadas.

language-es = Español
language-en = English

common-back = Volver
reveal-show = Mostrar contraseña
reveal-hide = Ocultar contraseña
error-generic = Algo salió mal: { $details }

dialog-open-title = Abrir una bóveda de KeePass
dialog-create-title = Guardar tu nueva bóveda
dialog-filter = Bóveda de KeePass

## Welcome

welcome-create = Crear una bóveda nueva
welcome-open = Abrir un archivo .kdbx
welcome-import-drive = Importar desde Google Drive
welcome-coming-soon = Próximamente
welcome-recent = Volver a las bóvedas recientes

## Create

create-title = Crear una bóveda
create-subtitle = Tu bóveda es un archivo estándar de KeePass, cifrado con tu contraseña maestra.
create-name = Nombre de la bóveda
create-name-default = Mi bóveda
create-password = Contraseña maestra
create-confirm = Confirmar contraseña maestra
create-warning = Una contraseña maestra olvidada no se puede recuperar. Guárdala en un lugar seguro.
create-submit = Elegir ubicación y crear
create-busy = Protegiendo tu bóveda…
create-error-name = Ponle un nombre a tu bóveda.
create-error-empty = Elige una contraseña maestra.
create-error-mismatch = Las contraseñas no coinciden.

strength-weak = Débil
strength-fair = Aceptable
strength-good = Buena
strength-strong = Fuerte
strength-bits = ~{ $bits } bits

## Unlock

unlock-title = Desbloquea tu bóveda
unlock-recent = Bóvedas recientes
unlock-select = Elige una bóveda para desbloquear.
unlock-password = Contraseña maestra
unlock-submit = Desbloquear
unlock-busy = Desbloqueando…
unlock-forget = Quitar de la lista
unlock-missing-file = Archivo no encontrado
unlock-open-other = Abrir otra bóveda
unlock-create = Crear una bóveda nueva
unlock-error-wrong-password = Contraseña maestra incorrecta.
unlock-error-not-found = No se encontró el archivo de la bóveda.

## Unlocked vault (placeholder until the main screen lands)

vault-unlocked = Bóveda desbloqueada
vault-entries =
    { $count ->
        [one] 1 entrada
       *[other] { $count } entradas
    }
vault-coming-soon = La pantalla principal llega en el próximo hito.
vault-lock = Bloquear
