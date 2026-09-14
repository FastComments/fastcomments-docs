[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Henter indlæg i et feed, nyeste først. Paginering er cursor-baseret: send `_id` for det sidste indlæg, du modtog, som `afterId` for at få den næste side.

Koster én kredit per ti indlæg, der returneres, med et minimum på én kredit.

[inline-code-attrs-start title = 'FeedPost cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Returner indlæg efter dette indlægs-id. Udelad for den første side. **/
    afterId?: string
    /** Standard er 10. Maksimum 1000. **/
    limit?: number
    /** Returner kun indlæg med disse tags. Gentag parameteren for mere end ét tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Inkluderet ved fejl. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]