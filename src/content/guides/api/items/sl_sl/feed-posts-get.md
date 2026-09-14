[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Pridobi objave v viru, najnovejše najprej. Straničenje je zasnovano na kazalcu: posredujte `_id` zadnje prejete objave kot
`afterId`, da dobite naslednjo stran.

Strošek je en kredit na deset vrnjenih objav, z minimalnim stroškom enega kredita.

[inline-code-attrs-start title = 'Primer cURL za FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Vrni objave po tem ID-ju objave. Izpusti za prvo stran. **/
    afterId?: string
    /** Privzeto 10. Največ 1000. **/
    limit?: number
    /** Vrni samo objave s temi oznakami. Parameter ponovite za več kot eno oznako. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Vključeno ob napaki. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---