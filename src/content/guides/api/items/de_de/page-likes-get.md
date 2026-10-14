[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Gibt die Anzahl der Likes auf einer Seite zurück und ob der aktuelle Benutzer sie geliked hat. Seiten, die noch nicht existieren, geben einen `likeCount` von `0` zurück.

Verwenden Sie `includeUserInfo=public`, um ebenfalls die Benutzer zu erhalten, die die Seite geliked haben, wobei die neuesten zuerst angezeigt werden. Benutzer mit privater Profilaktivität und anonyme Benutzer werden ausgeschlossen, und es werden bis zu 100 Likes abgefragt. Hier wird nur `public` akzeptiert; `include-private` liefert einen `422` und ist über [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) mit einem API‑Schlüssel verfügbar. Auf Seiten, die SSO erfordern, gibt `includeUserInfo` einen `403` zurück für Benutzer, die die Seite nicht anzeigen können.

[inline-code-attrs-start title = 'Page Likes cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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