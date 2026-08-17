[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, обезбеђену параметром упита `skip`. TenantPackages се враћају у страницама од `100`, поређани по `createdAt` и `id`.

Трошак се заснива на броју враћених tenant packages, при чему кошта `1 кредит по 10` враћених tenant packages.

[inline-code-attrs-start title = 'TenantPackage cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број tenant packages које треба прескочити за пагинацију. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају грешке. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]