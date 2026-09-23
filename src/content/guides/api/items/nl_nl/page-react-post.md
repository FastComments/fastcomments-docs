[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Voegt een reactie toe aan een pagina als de huidige gebruiker. Een gebruiker kan elke reactie-id één keer toevoegen: het opnieuw toevoegen slaagt met de code `already-reacted` en verandert de telling niet. Een gebruiker kan verschillende reacties aan dezelfde pagina toevoegen.

Reactie-id's worden door jou gekozen en mogen tot 36 tekens bevatten. De pagina wordt aangemaakt als deze nog niet bestaat. Geef `title` door om de titel van de pagina in te stellen of bij te werken.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor Pagina Reactie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Reactie Verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Reactie Respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]