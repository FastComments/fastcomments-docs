[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ta API uporablja paginacijo, ki jo zagotavlja poizvedni parameter `skip`. TenantUsers se vračajo v straneh po `100`, urejeni po `signUpDate`, `username` in `id`.

Cena je odvisna od števila vrnjenih tenant uporabnikov; znaša `1 credit per 10` vrnjenih tenant uporabnikov.

[inline-code-attrs-start title = 'Primer cURL zahteve za TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Število tenant uporabnikov, ki jih je treba preskočiti pri paginaciji. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---