[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Esta API usa paginação, fornecida pelo parâmetro de consulta `skip`. TenantUsers são retornados em páginas de `100`, ordenados por `signUpDate`, `username` e `id`.

O custo é baseado no número de TenantUsers retornados, custando `1 credit per 10` TenantUsers retornados.

[inline-code-attrs-start title = 'Exemplo cURL de TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O número de TenantUsers a pular para paginação. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---