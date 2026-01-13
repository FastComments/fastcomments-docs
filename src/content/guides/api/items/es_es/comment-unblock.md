[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de desbloquear a un usuario que escribió un comentario dado. Soporta el desbloqueo de comentarios escritos por Usuarios de FastComments.com, Usuarios SSO y Usuarios de Inquilino.

Soporta un parámetro de cuerpo `commentIdsToCheck` para verificar si algún otro comentario potencialmente visible en el cliente debe ser bloqueado/desbloqueado después de realizar esta acción.

Notas:

- Esta llamada siempre debe hacerse en el contexto de un usuario. El usuario puede ser un Usuario de FastComments.com, Usuario SSO o Usuario de Inquilino.
- El `userId` en la solicitud es el usuario que *está haciendo el desbloqueo*. Por ejemplo: `Usuario A` quiere Desbloquear a `Usuario B`. Pase `userId=Usuario A` y el id del comentario que escribió `Usuario B`.
- Los comentarios completamente anónimos (sin id de usuario, sin email) no pueden ser bloqueados y se devolverá un error.

[inline-code-attrs-start title = 'Ejemplo cURL de Desbloqueo de Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Ejemplo cURL de Desbloqueo de Comentario Anónimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Desbloqueo de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Desbloqueo de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
