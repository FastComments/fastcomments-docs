[api-resource-header-start name = 'Page'; route = 'DELETE /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Deze route verwijdert een enkele pagina op basis van het id.

Let op dat interactie met de reactie-widget voor een pagina met hetzelfde `urlId` de `Page` naadloos opnieuw zal aanmaken.

[inline-code-attrs-start title = 'cURL-voorbeeld voor Paginaverwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pages/some-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het verzoek voor pagina-verwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het antwoord bij pagina-verwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'page-does-not-exist'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]