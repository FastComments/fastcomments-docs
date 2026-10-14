[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Gibt die Anzahl für jede Reaktion auf einer Seite zurück und optional die Benutzer, die reagiert haben, als ein Eintrag pro Reaktion mit seiner Reaktions‑ID, neueste zuerst. Dies ist die serverseitige Version von [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` steuert, welche Benutzer zurückgegeben werden:

- `public` lässt Benutzer weg, die ihre Profilaktivität privat gemacht haben.
- `include-private` schließt sie ein. Verwenden Sie dies nur, wenn die Liste nicht öffentlich angezeigt wird.

Anonyme Benutzer werden niemals einbezogen, und bis zu 100 Reaktionen werden abgefragt. Seiten, die nicht existieren, geben keine `counts` zurück und eine leere `users`‑Liste.

[inline-code-attrs-start title = 'Page Reacts cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Hilfreicher Hinweis

Denken Sie daran, Werte wie die `urlId` URI‑zu kodieren.