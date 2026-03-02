La página de configuración del complemento está en **Administración del sitio > Complementos > Complementos locales > FastComments**. Las opciones disponibles son:

#### Tenant ID

Su Tenant ID de FastComments. Encuéntrelo en el <a href="https://fastcomments.com/auth/my-account" target="_blank">panel de control de FastComments</a> en la configuración de su cuenta.

#### API Secret

Su clave API Secret, requerida para el modo SSO Secure. Encuéntrela en <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mi cuenta > API Secret</a>.

#### SSO Mode

Elija cómo se autentican los usuarios. Vea la sección [Modos SSO](#moodle-sso-modes) para detalles sobre cada opción.

- **Secure** (recomendado) - autenticación firmada en el servidor con HMAC-SHA256
- **Simple** - datos de usuario en el cliente sin firma
- **None** - comentarios anónimos, sin integración con el inicio de sesión de Moodle

#### Page Contexts

Controla dónde aparecen los comentarios:

- **Course pages** - comentarios en las páginas principales del curso
- **Module/activity pages** - comentarios en actividades y recursos individuales
- **Both** - comentarios en todos los tipos de página

#### Commenting Style

Elija la experiencia de comentarios. Vea [Estilos de comentarios](#moodle-commenting-styles) para capturas de pantalla de cada modo.

- **Comments** - widget estándar de comentarios en hilos debajo del contenido de la página
- **Collab Chat** - discusiones en línea mediante selección de texto con indicadores de presencia
- **Both** - comentarios y collab chat activos simultáneamente

#### CDN URL

La URL de CDN de FastComments. Por defecto `https://cdn.fastcomments.com`. Cambie esto a la URL de CDN de la UE si sus datos están alojados en la región de la UE.