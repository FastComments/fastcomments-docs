[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Restituisce il numero di like su una pagina e se l'utente corrente ha messo like. Le pagine che non esistono ancora restituiscono un `likeCount` di `0`.

[inline-code-attrs-start title = 'Esempio cURL per Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** JSON codificato URI del tuo oggetto SSO. Ometti per utenti anonimi. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: string
    /** Incluso in caso di errore. **/
    reason?: string
    likeCount: number
    /** Indica se l'utente che effettua la richiesta ha messo like alla pagina. **/
    didLike: boolean
    /** Il numero di commenti di primo livello nella pagina. **/
    commentCount: number
    /** L'ID usato per iscriversi agli aggiornamenti in tempo reale per questa pagina. **/
    urlIdWS: string
}
[inline-code-end]