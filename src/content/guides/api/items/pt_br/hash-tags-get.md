[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Esta API usa paginação, fornecida pelo parâmetro de consulta `page`. HashTags são retornadas em páginas de `100`, ordenadas por `tag`.

[inline-code-attrs-start title = 'Exemplo cURL de HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** A página a ser buscada, começando em 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    /** As hashtags! **/
    hashTags: HashTag[]
}
[inline-code-end]