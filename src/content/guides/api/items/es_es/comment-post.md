[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de crear comentarios.

Los casos de uso comunes son interfaces personalizadas, integraciones o importaciones.

Notas:

- Esta API puede actualizar el widget de comentarios "en vivo" si se desea (esto aumenta `creditsCost` de `1` a `2`).
- Esta API creará automáticamente objetos de usuario en nuestro sistema si se proporciona email.
- Intentar guardar dos comentarios con diferentes emails, pero el mismo nombre de usuario, resultará en un error para el segundo comentario.
- Si está especificando `parentId`, y un comentario hijo tiene `notificationSentForParent` como false, **enviaremos notificaciones para el comentario padre**. Esto se hace cada hora (agrupamos las notificaciones para disminuir el número de emails enviados).
- Si desea enviar emails de bienvenida al crear usuarios, o emails de verificación de comentarios, establezca `sendEmails` a `true` en los parámetros de consulta.
- Los comentarios creados a través de esta API aparecerán en las páginas de Análisis y Moderación de la aplicación de administración.
- Las "malas palabras" aún se enmascaran en los nombres de los comentaristas y el texto del comentario si la configuración está activada.
- Los comentarios creados a través de esta API aún pueden ser verificados para spam si se desea.
- La configuración como longitud máxima del comentario, si se configura a través de la página de administración de Reglas de Personalización, se aplicará aquí.

Los datos mínimos requeridos para enviar que se mostrarán en el widget de comentarios, son los siguientes:

[inline-code-attrs-start title = 'Ejemplo cURL Mínimo de POST de Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Una solicitud más realista podría verse así:

[inline-code-attrs-start title = 'Ejemplo cURL de POST de Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de POST de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de POST de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
