[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de la API permite eliminar un comentario.

Notas:

- Esta API puede actualizar el widget de comentarios "live" si se desea (esto aumenta el `creditsCost` de `1` a `2`).
- Esta API eliminará todos los comentarios secundarios.
- Si el comentario objetivo está bloqueado (`isLocked: true`), la solicitud se rechaza con `code: 'locked'`. Desbloquee el comentario primero, luego elimínelo.

[inline-code-attrs-start title = 'Ejemplo cURL de eliminación de comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Estructura de solicitud DELETE de comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** El usuario que realiza la actualización. Si se desea, puede usarse para verificar que puede eliminar el comentario.  **/
    contextUserId?: string
	/** Si el comentario debe eliminarse "live" para los usuarios que estén viendo instancias del widget de comentarios con el mismo urlId. NOTA: Duplica el costo de créditos de 1 a 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta DELETE de comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de fallo. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Incluido en caso de fallo. **/
    reason?: string
}
[inline-code-end]