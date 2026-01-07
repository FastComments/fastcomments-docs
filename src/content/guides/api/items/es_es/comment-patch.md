[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de actualizar un único comentario.

Notas:

- Esta API puede actualizar el widget de comentarios "en vivo" si se desea (esto aumenta el `creditsCost` base de `1` a `2`).
  - Esto puede hacer que la migración de comentarios entre páginas sea "en vivo" (cambiando `urlId`).
  - Las migraciones cuestan `2` créditos adicionales ya que las páginas son precalculadas y esto es intensivo en CPU.
- A diferencia de la API de creación, esta API NO creará automáticamente objetos de usuario en nuestro sistema si se proporciona email.
- Los comentarios actualizados a través de esta API aún pueden ser verificados para spam si se desea.
- La configuración como longitud máxima del comentario, si se configura a través de la página de administración de Reglas de Personalización, se aplicará aquí.
- Para permitir a los usuarios actualizar el texto de su comentario, puede simplemente especificar `comment` en el cuerpo de la solicitud. Generaremos el `commentHTML` resultante.
  - Si define tanto `comment` como `commentHTML` no generaremos automáticamente el HTML.
  - Si el usuario agrega menciones o hashtags en su nuevo texto, se procesará como la API `POST`.
- Cuando actualice `commenterEmail` en un comentario, es mejor también especificar `userId`. De lo contrario, debe asegurarse de que el usuario con este email pertenezca a su inquilino, o la solicitud fallará.


[inline-code-attrs-start title = 'Ejemplo cURL Mínimo de PATCH de Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de PATCH de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de PATCH de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
