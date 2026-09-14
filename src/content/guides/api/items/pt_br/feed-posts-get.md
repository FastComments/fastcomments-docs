[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Busca posts em um feed, mais recentes primeiro. A paginação é baseada em cursor: passe o `_id` do último post que você recebeu como
`afterId` para obter a próxima página.

Custa um crédito por cada dez posts retornados, com um mínimo de um crédito.

[inline-code-attrs-start title = 'Exemplo cURL do FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição do FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Retorna posts após este ID de post. Omitir para a primeira página. **/
    afterId?: string
    /** Padrão 10. Máximo 1000. **/
    limit?: number
    /** Retorna apenas posts com estas tags. Repita o parâmetro para mais de uma tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta do FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Incluído em caso de falha. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---