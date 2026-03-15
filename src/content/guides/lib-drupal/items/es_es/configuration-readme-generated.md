Navegue a **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Configuración

- **Tenant ID** (requerido) - Su Tenant ID de FastComments. Encuéntrelo en [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requerido para Secure SSO, verificación de webhooks y sincronización de páginas. Se encuentra en [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Integración Single Sign-On:
  - **None** - Sin SSO, los usuarios comentan como invitados o crean cuentas de FastComments.
  - **Simple** - Envía la información del usuario de Drupal (name, email, avatar) a FastComments sin verificación del lado del servidor.
  - **Secure** - Utiliza la verificación HMAC-SHA256 para autenticar de forma segura a los usuarios de Drupal con FastComments (recomendado).
- **Commenting Style** - El tipo de widget para mostrar:
  - **Live Comments** - Comentarios en tiempo real y en hilos.
  - **Streaming Chat** - Interfaz de chat en vivo.
  - **Collab Chat** - Anotación colaborativa mediante selección de texto en el área de contenido principal.
  - **Collab Chat + Comments** - Tanto collab chat como comentarios estándar.
- **CDN URL** - URL del CDN de FastComments (predeterminado: `https://cdn.fastcomments.com`).
- **Site URL** - URL del sitio de FastComments (predeterminado: `https://fastcomments.com`).
- **Email notifications** - Enviar un correo electrónico a los autores del contenido cuando se publique un nuevo comentario en su contenido.

### Añadir comentarios a tipos de contenido

Agregue el campo **FastComments** a sus tipos de contenido vía **Structure > Content types > [type] > Manage fields**. El campo tiene un interruptor de estado y un identificador personalizado opcional por entidad.

### Residencia de datos en la UE

Para la residencia de datos en la UE, actualice:
- **CDN URL** a `https://cdn-eu.fastcomments.com`
- **Site URL** a `https://eu.fastcomments.com`