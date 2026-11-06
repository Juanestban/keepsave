app-name = KeepSave
app-tagline = Tus contraseñas, cifradas y sincronizadas.

language-es = Español
language-en = English

common-back = Volver
reveal-show = Mostrar contraseña
reveal-hide = Ocultar contraseña
error-generic = Algo salió mal: { $details }
error-hello = Windows Hello falló: { $details }
hello-unavailable = Windows Hello no está configurado en este equipo.
hello-cancelled = Se canceló Windows Hello.

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
unlock-error-hello-stale = Windows Hello ya no puede abrir esta bóveda. Escribe tu contraseña maestra.
unlock-hello = Desbloquear con Windows Hello
unlock-hello-or = o usa tu contraseña maestra
unlock-hello-remember = Usar Windows Hello tras bloquear

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
drive-unavailable = Google Drive aún no está configurado.
drive-setup = Configurar Google Drive
drive-setup-hint = KeepSave se conecta a Drive con un cliente privado de Google que creas una sola vez. Una guía paso a paso te acompaña.
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
settings-hello = Windows Hello
settings-hello-hint = Después de desbloquear una bóveda con su contraseña maestra, Windows Hello puede volver a desbloquearla hasta que cierres KeepSave.
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
settings-drive-setup-hint = Crea tu propio cliente de Google con la guía. Son unos 10 minutos.
settings-drive-client = Cliente de Google
settings-drive-client-user = Tu propio cliente
settings-drive-client-user-hint = Guardado en el almacén de credenciales del sistema, nunca en el archivo de ajustes.
settings-drive-client-change = Cambiar
settings-drive-client-remove = Quitar el cliente y desconectar
settings-language-system = Sistema ({ $language })
settings-animated-background = Fondo animado
settings-animated-background-hint = Si está apagado, el fondo queda quieto y gasta menos energía.

## About

about-version = Versión { $version }
about-license = Software libre bajo la licencia MIT o Apache 2.0.
about-repository = Código fuente en GitHub
about-credits = Créditos
about-credit-inter = Tipografía Inter de Rasmus Andersson, bajo la SIL Open Font License 1.1.
about-credit-phosphor = Iconos Phosphor, bajo la licencia MIT.
about-credit-eff = Lista de palabras para frases de paso de la Electronic Frontier Foundation, bajo CC BY 3.0.
about-eff-link = Listas de palabras de la EFF

about-license-title = Licencia
about-updates = Actualizaciones

## Updates

update-available = Actualización disponible
update-ready = Actualización lista para instalar
update-versions = KeepSave { $version } · tienes la { $current }
update-close = Ocultar hasta el próximo inicio
update-install = Actualizar
update-notes = Novedades
update-downloading = Descargando… { $percent } %
update-restart = Reiniciar y actualizar
update-restart-hint = Si hay un vault abierto, primero se guarda y se bloquea.
update-retry = Reintentar
update-open-page = Página de la versión
update-check-now = Buscar ahora
update-automatic = Buscar actualizaciones automáticamente
update-automatic-hint = Al iniciar y cada 6 horas. Solo pregunta a GitHub cuál es la última versión.
update-status-idle = Aún no se ha comprobado.
update-status-checking = Buscando actualizaciones…
update-status-up-to-date = KeepSave está al día.
update-status-available = La versión { $version } está disponible.
update-status-downloading = Descargando la actualización…
update-error-network = No se pudo conectar con GitHub. Revisa tu conexión.
update-error-rate-limit = GitHub está limitando las peticiones. Inténtalo más tarde.
update-error-checksum = La descarga está dañada. Inténtalo de nuevo.
update-error-missing = Esta versión no se puede instalar desde la app. Descárgala desde la página de la versión.
update-error-folder = KeepSave no puede escribir en su carpeta. Descarga la actualización desde la página de la versión.


## Google Drive setup guide

gsetup-title = Configurar Google Drive
gsetup-close = Cerrar la guía
gsetup-time = Unos 10 minutos
gsetup-step-of = Paso { $step } de { $total }
gsetup-start = Empezar
gsetup-next = Siguiente
gsetup-back = Atrás
gsetup-save = Guardar y conectar
gsetup-open-cloud = Abrir en Google Cloud
gsetup-full-guide = Leer la guía completa en GitHub
gsetup-copied = Copiado
gsetup-chip-project = Nombre del proyecto
gsetup-chip-app = Nombre de la app
gsetup-chip-scope = Permiso
gsetup-chip-client = Nombre
gsetup-client-id = ID de cliente
gsetup-client-secret = Secreto del cliente
gsetup-client-id-invalid = Un ID de cliente termina en .apps.googleusercontent.com.

gsetup-intro-title = Tu propia conexión privada con Drive
gsetup-intro-body = KeepSave habla con Google Drive mediante un cliente de Google que es tuyo, así que no interviene la app de nadie más. Lo creas una vez, gratis, en Google Cloud.
gsetup-intro-1 = Necesitas una cuenta de Google. Usa aquella cuyo Drive guardará tus vaults.
gsetup-intro-2 = Deja esta ventana abierta. Cada paso tiene un botón que abre la página correcta de Google Cloud.
gsetup-intro-3 = Haz clic en un valor para copiarlo y pégalo donde indique el paso.
gsetup-intro-tip = Tu vault se cifra antes de salir de este PC, y KeepSave solo puede ver los archivos que crea en tu Drive.

gsetup-project-title = Crea un proyecto
gsetup-project-body = Un proyecto guarda la configuración de tu cliente de Google.
gsetup-project-1 = Llama al proyecto KeepSave y haz clic en Crear.
gsetup-project-2 = Espera la notificación de que el proyecto está listo.
gsetup-project-3 = Comprueba que KeepSave está seleccionado en el selector de proyectos de arriba.

gsetup-api-title = Activa la API de Google Drive
gsetup-api-body = Esto permite que tu proyecto hable con Google Drive.
gsetup-api-1 = Comprueba que el proyecto seleccionado arriba es KeepSave.
gsetup-api-2 = Haz clic en Habilitar y espera a que cargue la página de la API.

gsetup-branding-title = Describe la app
gsetup-branding-body = Es lo que Google muestra cuando inicias sesión desde KeepSave.
gsetup-branding-1 = Haz clic en Comenzar.
gsetup-branding-2 = Nombre de la app: KeepSave. Correo de asistencia: tu correo. Haz clic en Siguiente.
gsetup-branding-3 = Público: Externo. Haz clic en Siguiente.
gsetup-branding-4 = Correo de contacto: tu correo. Haz clic en Siguiente, acepta la política y haz clic en Crear.

gsetup-scope-title = Permite el acceso a los archivos de KeepSave
gsetup-scope-body = El permiso que KeepSave pedirá.
gsetup-scope-1 = Haz clic en Agregar o quitar permisos.
gsetup-scope-2 = En Agregar permisos manualmente, pega el permiso y haz clic en Agregar a la tabla.
gsetup-scope-3 = Haz clic en Actualizar y luego en Guardar, al final de la página.
gsetup-scope-tip = Este permiso deja que KeepSave vea solo los archivos que crea, nunca el resto de tu Drive.

gsetup-publish-title = Publica la app
gsetup-publish-body = Saca la app del modo de prueba para que la conexión no caduque.
gsetup-publish-1 = En Estado de publicación, haz clic en Publicar app.
gsetup-publish-2 = Confirma. El estado cambia a En producción.
gsetup-publish-tip = En modo de prueba, Google desconecta KeepSave cada 7 días. Este permiso no necesita la revisión de Google para publicar.

gsetup-client-title = Crea el cliente
gsetup-client-body = El cliente es el par de claves con el que KeepSave te hace iniciar sesión.
gsetup-client-1 = Tipo de aplicación: App de escritorio.
gsetup-client-2 = Nombre: KeepSave desktop. Haz clic en Crear.
gsetup-client-3 = Copia el ID de cliente y el secreto del cliente del cuadro que se abre.
gsetup-client-tip = Descargar JSON en ese cuadro guarda una copia. Guárdala en un lugar privado.

gsetup-paste-title = Pega tu cliente
gsetup-paste-body = KeepSave lo guarda en el almacén de credenciales del sistema, nunca en su archivo de ajustes.
gsetup-paste-tip = Al conectar, Google avisa de que no ha verificado la app. Es tu propia app: haz clic en Configuración avanzada y luego en Ir a KeepSave.

## Password generator

generator-open = Generador de contraseñas
generator-title = Generador de contraseñas
gen-copy = Copiar
