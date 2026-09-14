[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Holt Beiträge in einem Feed, neueste zuerst. Die Paginierung ist cursor‑basiert: übergebe die `_id` des letzten erhaltenen Beitrags als
`afterId`, um die nächste Seite zu erhalten.

Kosten: ein Credit pro zehn zurückgegebene Beiträge, mindestens ein Credit.

[inline-code-attrs-start title = 'FeedPost cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Gibt Beiträge nach dieser Beitrags-ID zurück. Für die erste Seite weglassen. **/
    afterId?: string
    /** Standardwert ist 10. Maximum 1000. **/
    limit?: number
    /** Gibt nur Beiträge mit diesen Tags zurück. Wiederhole den Parameter für mehr als einen Tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Bei einem Fehler enthalten. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]