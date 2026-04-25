[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de la API permite actualizar un único comentario.

Notas:

- Esta API puede actualizar el widget de comentarios "en vivo" si se desea (esto aumenta el `creditsCost` base de `1` a `2`).
  - Esto puede hacer que la migración de comentarios entre páginas sea "en vivo" (cambiando `urlId`).
  - Las migraciones cuestan `2` créditos adicionales ya que las páginas se precalculan y esto es intensivo en CPU.
- A diferencia de la API de creación, esta API NO creará automáticamente objetos de usuario en nuestro sistema si se proporciona el correo electrónico.
- Los comentarios actualizados mediante esta API aún pueden comprobarse por spam si se desea.
- La configuración, como la longitud máxima de comentario, si está configurada mediante la página de administración de la Regla de Personalización, se aplicará aquí.
- Para permitir que los usuarios actualicen el texto de su comentario, puede simplemente especificar `comment` en el cuerpo de la solicitud. Nosotros generaremos el `commentHTML` resultante.
  - Si define tanto `comment` como `commentHTML` no generaremos automáticamente el HTML.
  - Si el usuario añade menciones o hashtags en su nuevo texto, se procesarán igual que en la API `POST`.
- Al actualizar `commenterEmail` en un comentario, es aconsejable especificar también `userId`. De lo contrario, debe asegurarse de que el usuario con ese correo electrónico pertenezca a su tenant, o la solicitud fallará.  
- Si el comentario objetivo está bloqueado (`isLocked: true`), la solicitud se rechazará con `code: 'locked'`. Desbloquee el comentario primero, actualícelo y luego vuelva a bloquearlo si lo desea.


[inline-code-attrs-start title = 'Ejemplo mínimo de PATCH de comentario con cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud PATCH de comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** El usuario que realiza la actualización. Si se desea, puede usarse para comprobar que puede editar el comentario.  **/
    contextUserId?: string
	/** ¿Deberíamos comprobar si el nuevo comentario parece spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Si el comentario debe aparecer "en vivo" para los usuarios que ven instancias del widget de comentarios con el mismo urlId. NOTA: Duplica el coste de créditos de 1 a 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta PATCH de comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de fallo. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Incluido en caso de fallo. **/
    reason?: string
}
[inline-code-end]

---