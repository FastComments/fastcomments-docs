[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Retourneert de namen van de gebruikers die een reactie aan een pagina hebben toegevoegd, gesorteerd op alfabetische volgorde. Er worden maximaal 100 reacties opgezocht, en anonieme gebruikers worden niet meegenomen. Op pagina's die SSO vereisen, krijgen gebruikers die de pagina niet kunnen bekijken een `403` met de code `sso-required` of `page-forbidden`.

Om ook avatars en gebruikers‑ids te krijgen, gebruik [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) met `includeUserInfo=public`.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina React Gebruikers'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Verzoek voor Pagina React Gebruikers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** De reactie‑id. **/
    id: string
    /** URI‑gecodeerde JSON van uw SSO‑object. Laat weg voor anonieme gebruikers. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Reactie voor Pagina React Gebruikers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: string
    /** Opgenomen bij een fout. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]