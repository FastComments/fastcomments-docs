[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Verwijdert een van de reacties van de huidige gebruiker van een pagina. Als de gebruiker die reactie niet heeft toegevoegd, slaagt het verzoek met de code `no-react` en verandert de telling niet.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor paginareactie verwijderen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van paginareactie verwijderingsverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** De reactie-id. **/
    id: string
    /** URI-gecodeerde JSON van uw SSO-object. Laat weg voor anonieme gebruikers. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van paginareactie verwijderingsrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' wanneer de gebruiker deze reactie niet had toegevoegd. Anders opgenomen bij een fout. **/
    code?: 'no-react' | string
    /** Opgenomen bij een fout. **/
    reason?: string
}
[inline-code-end]

---