[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Esta API usa paginação, fornecida pelo parâmetro de query `skip`. TenantPackages são retornados em páginas de `100`, ordenados por `createdAt` e `id`.

O custo é baseado no número de tenant packages retornados, custando `1 credit per 10` tenant packages retornados.

[inline-code-attrs-start title = 'Exemplo cURL de TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O número de pacotes de tenant a serem ignorados para paginação. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---