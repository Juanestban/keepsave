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

## Main screen

entries-count =
    { $count ->
        [one] 1 entrada
       *[other] { $count } entradas
    }
main-search = Buscar entradas (Ctrl+F)
main-search-clear = Borrar búsqueda
main-new-entry = Nueva entrada
main-lock = Bloquear (Ctrl+L)
main-groups = Grupos
main-new-group = Nuevo grupo
main-recycle-bin = Papelera
main-results = Resultados para “{ $query }”
main-empty-group = Aún no hay entradas aquí.
main-empty-bin = La papelera está vacía.
main-no-results = Nada coincide con tu búsqueda.
main-copy-username = Copiar usuario (Ctrl+B)
main-copy-password = Copiar contraseña (Ctrl+C)

group-rename = Cambiar nombre
group-delete = Mover a la papelera
bin-empty = Vaciar papelera

status-saved = Guardado
status-saving = Guardando…
status-failed = Sin guardar
status-retry = Haz clic para reintentar. { $details }

toast-copied-username = Usuario copiado
toast-copied-password = Contraseña copiada
toast-copied-url = URL copiada
toast-copied-field = { $field } copiado
toast-clears-in = se borra en { $seconds } s
toast-copy-failed = No se pudo acceder al portapapeles.
toast-moved-to-bin = Movido a la papelera
toast-restored = Restaurado
toast-version-restored = Versión restaurada
toast-url-invalid = Solo se pueden abrir enlaces web (http o https).
toast-saving-before-lock = Guardando antes de bloquear…
toast-lock-save-failed = No se pudo guardar, así que la bóveda sigue desbloqueada.

field-title = Título
field-username = Usuario
field-password = Contraseña
field-url = URL
field-notes = Notas
field-tags = Etiquetas
field-tags-hint = Separa las etiquetas con comas.
field-group = Grupo
field-custom = Campos personalizados
field-custom-empty = Sin campos personalizados. Añade uno para cosas como un PIN o códigos de recuperación.
field-add = Añadir campo
field-name = Nombre del campo
field-protect = Proteger este valor
field-unprotect = Guardar como texto normal
field-remove = Quitar campo

detail-untitled = (sin título)
detail-edit = Editar (Enter)
detail-delete = Mover a la papelera (Supr)
detail-delete-forever = Eliminar definitivamente
detail-restore = Sacar de la papelera
detail-close = Cerrar (Esc)
detail-copy = Copiar
detail-open-url = Abrir URL (Ctrl+U)
detail-modified = Modificado { $date }
detail-history = Historial ({ $count })
detail-history-restore = Restaurar

edit-new-title = Nueva entrada
edit-title = Editar entrada
edit-save = Guardar
edit-cancel = Cancelar
edit-error-field-empty = Cada campo personalizado necesita un nombre.
edit-error-field-duplicate = Hay dos campos personalizados llamados “{ $name }”.
edit-error-field-reserved = “{ $name }” es un nombre reservado. Elige otro.

gen-toggle = Generar
gen-hide = Ocultar generador
gen-chars = Caracteres
gen-words = Frase
gen-length = Longitud
gen-word-count = Palabras
gen-separator = Separador
gen-ambiguous = Evitar caracteres parecidos (0 O 1 l I)
gen-regenerate = Generar otra
gen-use = Usar esta contraseña
gen-error-sets = Elige al menos un tipo de carácter.

dialog-cancel = Cancelar
dialog-new-group-title = Nuevo grupo
dialog-rename-group-title = Cambiar nombre del grupo
dialog-group-error-empty = Ponle un nombre al grupo.
dialog-create = Crear
dialog-rename = Cambiar nombre
dialog-delete-forever-title = ¿Eliminar definitivamente?
dialog-delete-forever-body = “{ $title }” se eliminará para siempre. No se puede deshacer.
dialog-delete = Eliminar
dialog-empty-bin-title = ¿Vaciar la papelera?
dialog-empty-bin-body = Todo lo que hay en la papelera se eliminará para siempre. No se puede deshacer.
dialog-empty = Vaciar
dialog-discard-title = ¿Descartar los cambios?
dialog-discard-body = Los cambios en esta entrada no se han guardado.
dialog-discard = Descartar
dialog-keep-editing = Seguir editando
dialog-close-title = ¿Cerrar sin guardar?
dialog-close-body = No se pudo guardar la bóveda. { $details }
dialog-close-anyway = Cerrar de todos modos
dialog-stay = Quedarme

## Google Drive

drive-title = Importar desde Google Drive
drive-subtitle = Elige una bóveda de la carpeta KeepSave de tu Drive.
drive-unavailable = Google Drive no está configurado en esta compilación.
drive-connect = Conectar Google Drive
drive-connect-short = Conectar Drive
drive-connect-hint = Se abrirá una pestaña del navegador para que inicies sesión en Google. KeepSave solo puede ver los archivos que crea.
drive-waiting = Esperando al navegador…
drive-cancel = Cancelar
drive-cancel-tooltip = Haz clic para cancelar
drive-loading = Buscando bóvedas…
drive-picking = Elige dónde guardarla…
drive-downloading = Descargando { $name }…
drive-empty = Aún no hay bóvedas en la carpeta KeepSave de tu Drive.
drive-retry = Reintentar
drive-upload = Subir a Drive
drive-uploading = Subiendo…
drive-linked = En Google Drive
drive-linked-tooltip = Vinculada a “{ $name }” en Drive. Haz clic para sincronizar ahora.
drive-unlink = Desvincular esta bóveda
drive-disconnect = Desconectar Google Drive
drive-error-denied = No se concedió el acceso en el navegador.
drive-error-timeout = El inicio de sesión tardó demasiado. Inténtalo de nuevo.
drive-error-reauthorize = Google Drive se desconectó. Vuelve a conectarlo.
drive-error-network = No se pudo conectar con Google. Revisa tu conexión.
drive-browser-success-title = KeepSave está conectado
drive-browser-success-body = Ya puedes cerrar esta pestaña y volver a KeepSave.
drive-browser-denied-title = KeepSave no se conectó
drive-browser-denied-body = No se concedió el acceso. Ya puedes cerrar esta pestaña.
dialog-import-title = Guarda la bóveda en este PC
toast-drive-uploaded = Bóveda subida a Google Drive

## Sync

sync-now = Sincronizar ahora
sync-running = Sincronizando…
sync-done = Sincronizada a las { $time }
sync-pending = Subida pendiente
sync-offline = Esperando la red
sync-offline-tooltip = Tus cambios están guardados en este PC. KeepSave reintenta cada minuto, o haz clic para intentarlo ya.
sync-failed = Error al sincronizar
toast-drive-merged =
    { $count ->
        [one] 1 cambio desde Google Drive
       *[other] { $count } cambios desde Google Drive
    }
drive-error-missing = Esta bóveda ya no está en Google Drive. Desvincúlala y vuelve a subirla.
drive-error-password = La copia de Google Drive tiene otra contraseña maestra, así que no se tocó.

## Settings

settings-open = Ajustes
settings-title = Ajustes
settings-close = Cerrar
settings-tab-general = General
settings-tab-security = Seguridad
settings-tab-drive = Google Drive
settings-tab-language = Idioma
settings-tab-appearance = Apariencia
settings-tab-about = Acerca de
settings-config-error = No se pudieron guardar los ajustes: { $details }
settings-recent = Bóvedas recientes
settings-recent-hint = Quitar una bóveda de esta lista no borra su archivo.
settings-recent-empty = No hay bóvedas recientes.
settings-recent-remove = Quitar de la lista
settings-recent-clear = Vaciar la lista
settings-never = Nunca
settings-minutes = { $count } min
settings-seconds = { $count } s
settings-lock-after = Bloquear tras inactividad
settings-lock-after-hint = Tras este tiempo sin usar el ratón ni el teclado.
settings-lock-minimize = Bloquear al minimizar la ventana
settings-lock-sleep = Bloquear al suspender el PC o bloquear la sesión
settings-lock-sleep-windows = Por ahora solo en Windows.
settings-clipboard-clear = Borrar lo copiado tras
settings-clipboard-clear-hint = Solo si el portapapeles aún contiene lo que copió KeepSave.
settings-clipboard-keep = Nunca
settings-drive-connection = Conexión
settings-drive-connected = Conectado a Google Drive
settings-drive-disconnected = Sin conectar
settings-drive-disconnect = Desconectar
settings-drive-links = Bóvedas vinculadas
settings-drive-links-empty = Aún no hay bóvedas vinculadas. Abre una bóveda y elige “Subir a Drive” en su barra lateral.
settings-drive-unlink = Desvincular esta bóveda
settings-language-system = Sistema ({ $language })
settings-animated-background = Fondo animado
settings-animated-background-hint = Si está apagado, el fondo queda quieto y gasta menos energía.
settings-transparency = Transparencia del sistema
settings-transparency-hint = Deja ver el fondo Mica de Windows 11 a través de la ventana.
settings-transparency-unsupported = Requiere Windows 11 versión 22H2 o posterior.
settings-restart = Reinicia KeepSave para aplicar este cambio.

## About

about-version = Versión { $version }
about-license = Software libre bajo la licencia MIT o Apache 2.0.
about-repository = Código fuente en GitHub
about-credits = Créditos
about-credit-inter = Tipografía Inter de Rasmus Andersson, bajo la SIL Open Font License 1.1.
about-credit-phosphor = Iconos Phosphor, bajo la licencia MIT.
about-credit-eff = Lista de palabras para frases de paso de la Electronic Frontier Foundation, bajo CC BY 3.0.
about-eff-link = Listas de palabras de la EFF

## Password generator

generator-open = Generador de contraseñas
generator-title = Generador de contraseñas
gen-copy = Copiar
