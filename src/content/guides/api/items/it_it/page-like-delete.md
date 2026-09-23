[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Rimuove il like dell'utente corrente da una pagina. Se l'utente non ha messo like alla pagina, la richiesta ha successo con il codice `not-liked` e non modifica il conteggio.

[inline-code-attrs-start title = 'Esempio cURL per rimuovere il like della pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta Page Unlike'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** JSON del tuo oggetto SSO codificato in URI. Omettere per gli utenti anonimi. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Page Unlike'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' quando l'utente non aveva messo like alla pagina. Altrimenti incluso in caso di errore. **/
    code?: 'not-liked' | string
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]