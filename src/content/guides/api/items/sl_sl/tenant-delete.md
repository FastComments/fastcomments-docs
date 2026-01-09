[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Ta pot omogoča odstranitev `Tenant` **in vseh povezanih podatkov** (uporabniki, komentarji itd.) po id-ju.

The following restrictions exist around removing tenants:

- Najemnik mora biti vaš ali najemnik z belo etiketo, ki ga upravljate.
- Poizvedbeni parameter `sure` mora biti nastavljen na `true`.

[inline-code-attrs-start title = 'Primer cURL zahtevka za odstranitev najemnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtevka za odstranitev najemnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev najemnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]

---