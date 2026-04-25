All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Requerido

- **Tenant ID** - Su FastComments Tenant ID. Se encuentra en [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requerido para Secure SSO, verificación de webhooks y sincronización de páginas. Se encuentra en [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Estilo de comentarios

Elija el widget que coincida con la forma en que desea que la gente converse en su sitio.

- **Live Comments** - Comentarios en hilo en tiempo real.
- **Streaming Chat** - Interfaz de chat en directo, ideal para eventos y transmisiones en vivo.
- **Collab Chat** - Anotación mediante selección de texto en el área de contenido principal. Los visitantes resaltan texto y comienzan una discusión en contexto.
- **Collab Chat + Comments** - Tanto collab chat como comentarios estándar en la misma página.

## Modo SSO

- **None** - Sin SSO. Los usuarios comentan como invitados o crean una cuenta de FastComments.
- **Simple** - Pasa la información del usuario de Drupal (name, email, avatar) a FastComments sin verificación del lado del servidor.
- **Secure** - Utiliza HMAC-SHA256 para verificar usuarios de Drupal con FastComments. Recomendado cuando tenga configurado un API Secret.

Consulte la sección `Single Sign-On (SSO)` para más detalles.

## Otras configuraciones

- **CDN URL** - Por defecto `https://cdn.fastcomments.com`.
- **Site URL** - Por defecto `https://fastcomments.com`.
- **Email notifications** - Enviar un correo electrónico al autor del contenido cuando se publique un nuevo comentario en su contenido.

Para la residencia de datos en la UE, consulte la sección `EU Data Residency`.

---