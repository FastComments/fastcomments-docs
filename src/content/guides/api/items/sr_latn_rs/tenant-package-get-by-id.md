[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint vraća jedan Tenant Package po id-u.

[inline-code-attrs-start title = 'Primer cURL zahteva za TenantPackage po id-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    tenantPackage: TenantPackage
}
[inline-code-end]

---