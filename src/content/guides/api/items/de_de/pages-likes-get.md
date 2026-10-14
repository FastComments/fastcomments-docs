[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Gibt die Anzahl der Likes auf einer Seite zurück und optional die Benutzer, die sie geliked haben, zuletzt zuerst. Dies ist die serverseitige Version von [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` steuert, welche Benutzer zurückgegeben werden:

- `public` lässt Benutzer weg, die ihre Profilaktivität privat gestellt haben.
- `include-private` schließt sie ein. Verwenden Sie dies nur, wenn die Liste nicht öffentlich angezeigt wird.

Anonyme Benutzer werden niemals einbezogen, und bis zu 100 Likes werden abgefragt. Seiten, die nicht existieren, geben einen `likeCount` von `0` und eine leere `users`‑Liste zurück.

[inline-code-attrs-start title = 'Page Likes cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
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

#### Nützlicher Hinweis

Denken Sie daran, Werte wie die `urlId` URI‑zu kodieren.

---