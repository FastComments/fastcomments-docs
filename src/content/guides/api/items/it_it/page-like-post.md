[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Metti mi piace a una pagina come utente corrente. Ogni utente può mettere mi piace a una pagina una sola volta: mettere nuovamente mi piace ha successo con il codice `already-liked` e non cambia il conteggio.

La pagina viene creata se non esiste ancora. Passa `title` per impostare o aggiornare il titolo della pagina.

[inline-code-attrs-start title = 'Esempio cURL per Mettere Mi Piace alla Pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta di Mettere Mi Piace alla Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta di Mettere Mi Piace alla Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' when the user had already liked the page. Otherwise included on failure. **/
    code?: 'already-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]