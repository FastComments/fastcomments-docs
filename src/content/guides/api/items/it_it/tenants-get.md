[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Questa API restituisce i tenant che sono gestiti dal tuo tenant.

La paginazione è fornita dal parametro di query `skip`. I tenant vengono restituiti in pagine di `100`, ordinate per `signUpDate` e `id`.

Il costo è basato sul numero di tenant restituiti, pari a `1 credit per 10` tenant restituiti.

[inline-code-attrs-start title = 'Esempio cURL per Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Puoi definire parametri `meta` sugli oggetti `Tenant` e interrogare per trovare i tenant corrispondenti. Ad esempio, per la chiave `someKey` e il valore meta `some-value`, possiamo
costruire un oggetto JSON con questa coppia chiave/valore e poi codificarlo URI come parametro di query per filtrare:

[inline-code-attrs-start title = 'Esempio cURL - Query per Tenant tramite meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Il numero di tenant da saltare per la paginazione. **/
    skip?: number
    /** Filtra per parametri meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]