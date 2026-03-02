La página de configuración del complemento está en **Site Administration > Plugins > Local plugins > FastComments**. Las opciones disponibles son:

#### Tenant ID

Su FastComments Tenant ID. Encuéntrelo en el <a href="https://fastcomments.com/auth/my-account" target="_blank">panel de control de FastComments</a> en la configuración de su cuenta.

#### API Secret

Su API Secret, necesaria para el modo SSO Secure. Encuéntrelo en <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mi cuenta > API Secret</a>.

#### SSO Mode

Elija cómo se autentican los usuarios. Vea la sección [Modos SSO](#moodle-sso-modes) para detalles sobre cada opción.

- **Secure** (recomendado) - autenticación firmada HMAC-SHA256 del lado del servidor
- **Simple** - datos de usuario del lado del cliente sin firma
- **None** - comentarios anónimos, sin integración de inicio de sesión de Moodle

#### Page Contexts

Controla dónde aparecen los comentarios:

- **Course pages** - comentarios en las páginas principales del curso
- **Module/activity pages** - comentarios en actividades y recursos individuales
- **Both** - comentarios en todos los tipos de página

#### Commenting Style

Elija la experiencia de comentarios. Vea [Estilos de comentarios](#moodle-commenting-styles) para capturas de pantalla de cada modo.

- **Comments** - widget de comentarios en hilo estándar debajo del contenido de la página
- **Collab Chat** - discusiones en línea mediante selección de texto con indicadores de presencia
- **Both** - comentarios y Collab Chat activos simultáneamente

#### CDN URL

La FastComments CDN URL. Por defecto es `https://cdn.fastcomments.com`. Cambie esto al EU CDN URL si sus datos están alojados en la región de la UE.