[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Al momento puoi recuperare solo tutte le pagine (o una singola pagina tramite `/by-url-id`) associate al tuo account. Se desideri ricerche più dettagliate, [contattaci](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Esempio cURL per Pages'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta per Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Suggerimento utile

L'API `Comment` richiede un `urlId`. Puoi chiamare prima l'API `Pages`, per vedere quali valori `urlId` sono disponibili per te
e come appaiono.