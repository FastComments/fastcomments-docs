[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Diese API verwendet Paginierung, bereitgestellt durch den `skip`-Abfrageparameter. TenantPackages werden in Seiten von `100` zurückgegeben, sortiert nach `createdAt` und `id`.

Die Kosten basieren auf der Anzahl der zurückgegebenen Tenant-Pakete und betragen `1 Credit pro 10` zurückgegebene Tenant-Pakete.

[inline-code-attrs-start title = 'TenantPackage cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant packages to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
