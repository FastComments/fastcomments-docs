[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, omogućenu putem query parametra `skip`. TenantUsers se vraćaju u stranicama od `100`, poredani po `signUpDate`, `username` i `id`.

Trošak se zasniva na broju TenantUsers koji se vrate, i iznosi `1 credit per 10` vraćenih TenantUsers.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj tenant korisnika koje treba preskočiti za paginaciju. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju greške. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---