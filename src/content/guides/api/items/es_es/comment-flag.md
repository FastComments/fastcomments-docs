[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de marcar un comentario para un usuario específico.

Notas:

- Esta llamada siempre debe hacerse en el contexto de un usuario. El usuario puede ser un Usuario de FastComments.com, Usuario SSO o Usuario de Inquilino.
- Si se establece un umbral de marcar para ocultar, el comentario se ocultará automáticamente en vivo después de haber sido marcado el número definido de veces.
- Después de ser automáticamente desaprobado (oculto) - el comentario solo puede ser re-aprobado por un administrador o moderador. Desmarcar no re-aprobará el comentario.

[inline-code-attrs-start title = 'Ejemplo cURL de Marcar Comentario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Para marcar anónimamente, debemos especificar un `anonUserId`. Este puede ser un ID que representa la sesión anónima, o un UUID aleatorio.
Esto nos permite soportar marcar y desmarcar comentarios incluso si un usuario no ha iniciado sesión. De esta manera, el comentario puede ser marcado como
marcado cuando los comentarios se obtienen con el mismo `anonUserId`.

[inline-code-attrs-start title = 'Ejemplo cURL de Marcar Comentario Anónimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Estructura de Solicitud de Marcar Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Marcar Comentario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]
