[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Permite obtener votos dejados por un usuario en un `urlId` dado. Toma un `userId` que puede ser cualquier usuario de FastComments.com o `SSO User`.

Esto es útil si desea mostrar si un usuario ha votado en un comentario. Al obtener comentarios, simplemente llame a esta API al mismo tiempo para el usuario con el
mismo `urlId`.

Si está usando votación anónima, entonces querrá pasar `anonUserId` en su lugar.

[inline-code-attrs-start title = 'Ejemplo cURL de Votes para Usuario'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Ejemplo cURL de Votes para Usuario Anónimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Tenga en cuenta que los votos anónimos aparecerán en la lista `appliedAuthorizedVotes`. Se consideran autorizados ya que fueron creados a través de la API con una clave de API.

[inline-code-attrs-start title = 'Estructura de Solicitud de Votes para Usuario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Votes para Usuario'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
