La página de configuración del plugin se encuentra en **Site Administration > Plugins > Local plugins > FastComments**. Las opciones disponibles son:

#### Tenant ID

Su Tenant ID de FastComments. Encuéntrelo en el <a href="https://fastcomments.com/auth/my-account" target="_blank">panel de FastComments</a> bajo la configuración de su cuenta.

#### API Secret

Su clave API Secret, necesaria para el modo Secure SSO. Encuéntrela en <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Elija cómo se autentican los usuarios. Consulte la sección [SSO Modes](#moodle-sso-modes) para obtener detalles sobre cada opción.

- **Secure** (recomendado) - autenticación firmada del lado del servidor con HMAC-SHA256
- **Simple** - datos de usuario del lado del cliente sin firma
- **None** - comentarios anónimos, sin integración con el inicio de sesión de Moodle

#### Page Contexts

Controla dónde aparecen los comentarios:

- **Course pages** - comentarios en las páginas principales de los cursos
- **Module/activity pages** - comentarios en actividades y recursos individuales
- **Both** - comentarios en todos los tipos de páginas

#### Commenting Style

Elija la experiencia de comentarios. Consulte [Commenting Styles](#moodle-commenting-styles) para ver capturas de pantalla de cada modo.

- **Comments** - widget estándar de comentarios en hilo debajo del contenido de la página
- **Collab Chat** - discusiones en línea sobre texto seleccionado con indicadores de presencia
- **Both** - comentarios y chat colaborativo activos juntos

#### CDN URL

La URL del CDN de FastComments. Por defecto es `https://cdn.fastcomments.com`. Cámbiela a la URL del CDN de la UE si sus datos están alojados en la región de la UE.