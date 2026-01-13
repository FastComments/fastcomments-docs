[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Ta API uporablja paginacijo, ki jo zagotavlja poizvedni parameter `skip`. TenantPackages se vrnejo po `100` na stran, razvrščene po `createdAt` in `id`.

Stroški so odvisni od števila vrnjenih tenant packages; znaša `1 credit per 10` vrnjenih tenant packages.

[inline-code-attrs-start title = 'Primer cURL zahteve za TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Število tenant packages, ki jih je treba preskočiti pri paginaciji. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno v primeru napake. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]