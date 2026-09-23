[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Gibt die Anzahl der Likes auf einer Seite zurück und ob der aktuelle Benutzer sie geliked hat. Seiten, die noch nicht existieren, geben einen `likeCount` von `0` zurück.

[inline-code-attrs-start title = 'Beispiel für Page Likes cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Likes-Anfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Likes-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: string
    /** Bei einem Fehler enthalten. **/
    reason?: string
    likeCount: number
    /** Ob der Benutzer, der die Anfrage stellt, die Seite geliked hat. **/
    didLike: boolean
    /** Die Anzahl der obersten Kommentare auf der Seite. **/
    commentCount: number
    /** Die ID, die verwendet wird, um Live-Updates für diese Seite zu abonnieren. **/
    urlIdWS: string
}
[inline-code-end]

---