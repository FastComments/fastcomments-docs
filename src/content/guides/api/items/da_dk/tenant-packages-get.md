[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Dette API bruger paginering, leveret af `skip`-forespørgselsparameteren. TenantPackages returneres i sider af `100`, sorteret efter `createdAt` og `id`.

Omkostningen er baseret på antallet af returnerede tenant-pakker og koster `1 kredit pr. 10` returnerede tenant-pakker.

[inline-code-attrs-start title = 'TenantPackage cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant packages to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]
