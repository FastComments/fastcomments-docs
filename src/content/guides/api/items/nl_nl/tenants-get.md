[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Deze API retourneert tenants die door uw tenant worden beheerd.

Paginering wordt verzorgd door de queryparameter `skip`. Tenants worden geretourneerd in pagina's van `100`, gesorteerd op `signUpDate` en `id`.

De kosten zijn gebaseerd op het aantal geretourneerde tenants; het kost `1 credit per 10` geretourneerde tenants.

[inline-code-attrs-start title = 'Tenant cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

U kunt `meta`-parameters definiëren op de `Tenant`-objecten en zoeken naar overeenkomende tenants. Bijvoorbeeld, voor de sleutel `someKey` en de meta-waarde `some-value`, kunnen we
een JSON-object met deze sleutel/waarde maken en vervolgens URI-encoden als queryparameter om te filteren:

[inline-code-attrs-start title = 'Tenant Query op Meta cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Request-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Het aantal tenants om over te slaan voor paginering. **/
    skip?: number
    /** Filteren op meta-parameters. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Response-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij een fout. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---