[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Retourneert de namen van de gebruikers die een reactie aan een pagina hebben toegevoegd, gesorteerd op alfabetische volgorde. Tot 100 reacties worden opgezocht, en anonieme gebruikers en gebruikers met privéprofielactiviteit worden niet opgenomen. Op pagina's die SSO vereisen, krijgen gebruikers die de pagina niet kunnen bekijken een `403` met de code `sso-required` of `page-forbidden`.

Om ook avatars en gebruikers‑id's te krijgen, gebruik [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) met `includeUserInfo=public`.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina React Gebruikers'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina React Gebruikers Verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina React Gebruikers Respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]