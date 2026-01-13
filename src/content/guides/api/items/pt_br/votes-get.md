[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Os votos devem ser buscados por `urlId`.

### Tipos de Votos

Existem três tipos de votos:

- Authenticated Votes, que são aplicados ao comentário correspondente. Você pode criar esses via esta API.
- Authenticated Votes, que estão **pendentes** de verificação e, portanto, ainda não foram aplicados ao comentário. Esses são criados quando um usuário usa o mecanismo *login para votar* do FastComments.com.
- Anonymous Votes, que são aplicados ao comentário correspondente. Eles são criados juntamente com o comentário anônimo.

Estes são retornados em listas separadas na API para reduzir confusão.

[inline-code-attrs-start title = 'Exemplo cURL de Votos'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Votos'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Votos'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Votos autorizados e verificados, aplicados aos seus comentários correspondentes. **/
    appliedAuthorizedVotes: Vote[]
    /** Votos anônimos, aplicados aos seus comentários correspondentes. **/
    appliedAnonymousVotes: Vote[]
    /** Votos pendentes de verificação, ainda não aplicados aos seus comentários correspondentes. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Observações sobre Votos Anônimos

Observe que votos anônimos criados via esta API aparecerão na lista `appliedAuthorizedVotes`. Eles são considerados autorizados, uma vez que foram criados através da API com uma API key.

A estrutura `appliedAnonymousVotes` é para votos criados sem um email, API key, etc.

---