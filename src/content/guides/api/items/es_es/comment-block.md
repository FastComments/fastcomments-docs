[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de bloquear a un usuario que escribió un comentario dado. Soporta el bloqueo de comentarios escritos por Usuarios de FastComments.com, Usuarios SSO y Usuarios de Inquilino.

Soporta un parámetro de cuerpo `commentIdsToCheck` para verificar si algún otro comentario potencialmente visible en el cliente debe ser bloqueado/desbloqueado después de realizar esta acción.

Notas:

- Esta llamada siempre debe hacerse en el contexto de un usuario. El usuario puede ser un Usuario de FastComments.com, Usuario SSO o Usuario de Inquilino.
- El `userId` en la solicitud es el usuario que *está haciendo el bloqueo*. Por ejemplo: `Usuario A` quiere Bloquear a `Usuario B`. Pase `userId=Usuario A` y el id del comentario que escribió `Usuario B`.
- Los comentarios completamente anónimos (sin id de usuario, sin email) no pueden ser bloqueados y se devolverá un error.

[inline-code-attrs-start title = 'Ejemplo cURL de Bloqueo de Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Para bloqueo anónimo, debemos especificar un `anonUserId`. Este puede ser un ID que representa la sesión anónima, o un UUID aleatorio.
Esto nos permite soportar el bloqueo de comentarios incluso si un usuario no ha iniciado sesión obteniendo los comentarios con el mismo `anonUserId`.

[inline-code-attrs-start title = 'Ejemplo cURL de Bloqueo de Comentario Anónimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Bloqueo de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Bloqueo de Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
