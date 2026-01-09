[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, omogućenu pomoću upitnog parametra `skip`. TenantPackages se vraćaju u stranicama od `100`, posložene po `createdAt` i `id`.

Trošak se temelji na broju vraćenih tenant paketa, a iznosi `1 credit per 10` vraćenih tenant paketa.

[inline-code-attrs-start title = 'Primjer cURL-a za TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj tenant paketa koji treba preskočiti za paginaciju. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---