[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ta API vrne tenants, ki jih upravlja vaš tenant.

Paginacija se izvaja z parametrom poizvedbe `skip`. Tenants so vrnjeni v straneh po `100`, urejeni po `signUpDate` in `id`.

Strošek je odvisen od števila vrnjenih tenants in znaša `1 credit per 10` vrnjenih tenants.

[inline-code-attrs-start title = 'Primer cURL zahtevka za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Na objektih `Tenant` lahko določite parametre `meta` in poizvedujete po ujemajočih se tenants. Na primer, za ključ `someKey` in vrednost meta `some-value`, lahko sestavimo JSON objekt s tem parom ključ/vrednost in ga nato URI-enkodiramo kot parametar poizvedbe za filtriranje:

[inline-code-attrs-start title = 'Primer cURL poizvedbe Tenant po meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtevka Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Število tenants, ki jih je treba preskočiti za paginacijo. **/
    skip?: number
    /** Filtriraj po meta parametrih. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]