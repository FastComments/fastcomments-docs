[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Ova ruta omogućava uklanjanje `Tenant` **i svih pridruženih podataka** (korisnici, komentari, itd.) po id-u.

Sljedeća ograničenja postoje prilikom uklanjanja Tenanta:

- Tenant mora biti vaš, ili white-labeled tenant koji vi upravljate.
- Parametar upita `sure` mora biti postavljen na `true`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje Tenanta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]