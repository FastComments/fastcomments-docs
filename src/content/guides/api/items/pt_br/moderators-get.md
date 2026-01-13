[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Esta API usa paginação, fornecida pelo parâmetro de consulta `skip`. Moderadores são retornados em páginas de `100`, ordenados por `createdAt` e `id`.

O custo é baseado no número de moderadores retornados, custando `1 credit per 10` moderadores retornados.

[inline-code-attrs-start title = 'Exemplo cURL do Moderador'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição do Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O número de moderadores a pular para paginação. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta do Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---