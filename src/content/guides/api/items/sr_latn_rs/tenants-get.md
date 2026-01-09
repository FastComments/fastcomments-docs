[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ovaj API vraća tenante kojima upravlja vaš tenant.

Paginacija se obezbeđuje pomoću query parametra `skip`. Tenanti se vraćaju u stranicama po `100`, sortirani po `signUpDate` i `id`.

Trošak se zasniva na broju vraćenih tenanata, iznosi `1 credit per 10` vraćenih tenanata.

[inline-code-attrs-start title = 'Primer cURL zahteva za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Možete definisati `meta` parametre na `Tenant` objektima i upitavati za odgovarajuće tenante. Na primer, za ključ `someKey` i meta vrednost `some-value`, možemo konstruisati JSON objekat sa ovim parom ključ/vrednost i zatim ga URI enkodovati kao query parametar za filtriranje:

[inline-code-attrs-start title = 'Primer cURL zahteva za pretragu Tenant-a po meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj tenanata koji će biti preskočeni za paginaciju. **/
    skip?: number
    /** Filtriraj po meta parametrima. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]