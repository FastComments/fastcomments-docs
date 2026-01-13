[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Dette API returnerer tenants, der administreres af din tenant.

Paginering leveres af `skip`-forespørgselsparameteren. Tenants returneres i sider af `100`, sorteret efter `signUpDate` og `id`.

Omkostningen er baseret på antallet af returnerede tenants og koster `1 kredit pr. 10` returnerede tenants.

[inline-code-attrs-start title = 'Tenant cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Du kan definere `meta`-parametre på `Tenant`-objekterne og forespørge efter matchende tenants. For eksempel, for nøglen `someKey` og meta-værdien `some-value`, kan vi
konstruere et JSON-objekt med dette nøgle/værdi-par og derefter URI-kode det som en forespørgselsparameter for at filtrere:

[inline-code-attrs-start title = 'Tenant Forespørgsel efter Meta cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
