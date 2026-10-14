[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Restituisce il numero di like su una pagina e se l'utente corrente ha messo like. Le pagine che non esistono ancora restituiscono un `likeCount` di `0`.

Passa `includeUserInfo=public` per ottenere anche gli utenti che hanno messo like alla pagina, dal più recente al più vecchio. Gli utenti con attività di profilo privata e gli utenti anonimi sono esclusi, e vengono cercati fino a 100 like. Solo `public` è accettato qui; `include-private` restituisce un `422` ed è disponibile da [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) con una chiave API. Sulle pagine che richiedono SSO, `includeUserInfo` restituisce un `403` per gli utenti che non possono visualizzare la pagina.

[inline-code-attrs-start title = 'Esempio cURL di Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]