All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Requerido

- **Tenant ID** - Su FastComments Tenant ID. Encuéntrelo bajo [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requerido para Secure SSO, verificación de webhook y sincronización de página. Se encuentra bajo [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Almacenamiento del API Secret

El módulo lee el API Secret del primer origen de los siguientes que tenga un valor:

1. **Key module** - Instale [Key](https://www.drupal.org/project/key), cree una clave de *Authentication* y selecciónela bajo **API Secret key** en el formulario de configuración. La clave puede estar respaldada por una variable de entorno o un archivo fuera de la raíz web.
2. **settings.php** - Añada `$config['fastcomments.settings']['api_secret'] = 'your-secret';` a `settings.php` o `settings.local.php`. El secreto nunca entra en la configuración exportada.
3. **The settings form** - Ingréselo en el campo **API Secret**. Esto almacena el valor en `fastcomments.settings`, por lo que termina en las exportaciones de configuración. El informe de estado de Drupal advierte cuando esta es la fuente activa.

El formulario muestra qué fuente está activa. Dejar el campo vacío mantiene el secreto almacenado. Marque **Remove the API Secret stored in configuration** para eliminarlo después de mover el secreto al módulo Key o a `settings.php`.

## Estilo de Comentarios

Elija el widget que coincida con la forma en que desea que la gente hable en su sitio.

- **Live Comments** - Comentarios en hilos en tiempo real.
- **Streaming Chat** - Interfaz de chat en vivo, buena para eventos y transmisiones en directo.
- **Collab Chat** - Anotación mediante selección de texto en el área de contenido principal. Los visitantes resaltan texto y comienzan una discusión en contexto.
- **Collab Chat + Comments** - Tanto chat colaborativo como comentarios estándar en la misma página.

## Modo SSO

- **None** - Sin SSO. Los usuarios comentan como invitados o crean una cuenta de FastComments.
- **Simple** - Transmite la información del usuario de Drupal (nombre, correo electrónico, avatar) a FastComments sin verificación del lado del servidor.
- **Secure** - Utiliza HMAC-SHA256 para verificar usuarios de Drupal con FastComments. Recomendado cuando tiene un API Secret configurado.

Consulte la sección `Single Sign-On (SSO)` para obtener más detalles.

## Otras Configuraciones

- **CDN URL** - Por defecto `https://cdn.fastcomments.com`.
- **Site URL** - Por defecto `https://fastcomments.com`.
- **Email notifications** - Envía un correo electrónico al autor del contenido cuando se publica un nuevo comentario en su contenido.

Para la residencia de datos en la UE, consulte la sección `EU Data Residency`.