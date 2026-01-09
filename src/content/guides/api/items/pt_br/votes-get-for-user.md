[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Permite buscar votos deixados por um usuário em um determinado `urlId`. Recebe um `userId` que pode ser qualquer usuário do FastComments.com ou `SSO User`.

Isto é útil se você quiser mostrar se um usuário votou em um comentário. Ao buscar comentários, simplesmente chame esta API ao mesmo tempo para o usuário com o mesmo `urlId`.

Se você estiver usando votação anônima, então deverá passar `anonUserId` em vez disso.

[inline-code-attrs-start title = 'Exemplo cURL: Votos para Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL: Votos para Usuário Anônimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Observe que votos anônimos aparecerão na lista `appliedAuthorizedVotes`. Eles são considerados autorizados pois foram criados via a API com uma API key.

[inline-code-attrs-start title = 'Estrutura da Requisição: Votos para Usuário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta: Votos para Usuário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Votos autorizados e verificados, aplicados aos seus comentários correspondentes. **/
    appliedAuthorizedVotes: Vote[]
    /** Votos pendentes de verificação, ainda não aplicados aos seus comentários correspondentes. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---