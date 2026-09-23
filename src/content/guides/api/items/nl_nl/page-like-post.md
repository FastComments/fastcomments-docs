[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vindt een pagina leuk als de huidige gebruiker. Elke gebruiker kan een pagina één keer leuk vinden: opnieuw leuk vinden slaagt met de code `already-liked` en verandert de telling niet.

De pagina wordt aangemaakt als deze nog niet bestaat. Geef `title` door om de titel van de pagina in te stellen of bij te werken.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina Like'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Like Verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Stelt de titel van de pagina in. **/
    title?: string
    /** URI-gecodeerde JSON van uw SSO-object. Laat weg voor anonieme gebruikers. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Like Reactie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' wanneer de gebruiker de pagina al leuk vond. Anders opgenomen bij een fout. **/
    code?: 'already-liked' | string
    /** Opgenomen bij een fout. **/
    reason?: string
}
[inline-code-end]