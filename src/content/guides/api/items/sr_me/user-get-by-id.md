[api-resource-header-start name = 'User'; route = 'GET /api/v1/users/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta vraća jednog korisnika po ID-u.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za korisnika po ID-u'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UserByIdResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'unauthorized'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    user?: User
}
[inline-code-end]

---