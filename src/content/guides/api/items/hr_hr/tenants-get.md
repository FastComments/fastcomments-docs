[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ovaj API vraća tenants kojima upravlja vaš tenant.

Paginacija se obavlja pomoću query parametra `skip`. Tenants se vraćaju u stranicama po `100`, sortirano po `signUpDate` i `id`.

Trošak se temelji na broju vraćenih tenants-a, iznosi `1 credit per 10` vraćenih tenants-a.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Možete definirati `meta` parametre na `Tenant` objektima i upitom tražiti odgovarajuće tenants-e. Na primjer, za ključ `someKey` i meta vrijednost `some-value`, možemo konstruirati JSON objekt s tim parom ključ/vrijednost i onda ga URI enkodirati kao query parametar za filtriranje:

[inline-code-attrs-start title = 'Primjer cURL upita za Tenant po meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj tenants-a koje treba preskočiti za paginaciju. **/
    skip?: number
    /** Filtriraj po meta parametrima. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---