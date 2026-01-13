[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Los votos deben ser obtenidos por `urlId`.

### Tipos de Votos

Hay tres tipos de votos:

- Votos Autenticados, que se aplican al comentario correspondiente. Puede crear estos a través de esta API.
- Votos Autenticados, que están **pendientes** de verificación, y por lo tanto aún no se aplican al comentario. Estos se crean cuando un usuario usa el mecanismo de *iniciar sesión para votar* de FastComments.com.
- Votos Anónimos, que se aplican al comentario correspondiente. Estos se crean junto con los comentarios anónimos.

Estos se devuelven en listas separadas en la API para reducir la confusión.

[inline-code-attrs-start title = 'Ejemplo cURL de Votes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Notas sobre Votos Anónimos

Tenga en cuenta que los votos anónimos creados a través de esta API aparecerán en la lista `appliedAuthorizedVotes`. Se consideran autorizados ya que fueron creados a través de la API con una clave de API.

La estructura `appliedAnonymousVotes` es para votos creados sin un email, clave de API, etc.
