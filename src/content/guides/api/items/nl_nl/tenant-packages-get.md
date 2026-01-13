[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Deze API gebruikt paginering, geleverd via de queryparameter `skip`. TenantPackages worden in pagina's van `100` teruggegeven, gesorteerd op `createdAt` en `id`.

De kosten zijn gebaseerd op het aantal geretourneerde tenant packages: `1 credit per 10` tenant packages.

[inline-code-attrs-start title = 'TenantPackage cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Het aantal tenant packages om over te slaan voor paginering. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij mislukking. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---