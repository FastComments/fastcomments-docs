[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Preuzima postove u feedu, najnovije prvo. Paginiranje je zasnovano na kursoru: prosledite `_id` poslednjeg posta koji ste primili kao `afterId` da biste dobili sledeću stranicu.

Košta jedan kredit po deset vraćenih postova, sa minimalno jednim kreditom.

[inline-code-attrs-start title = 'Primer cURL zahteva za FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Vrati postove posle ovog ID-a posta. Izostavite za prvu stranicu. **/
    afterId?: string
    /** Podrazumevano je 10. Maksimum je 1000. **/
    limit?: number
    /** Vrati samo postove sa ovim oznakama. Ponovite parametar za više od jedne oznake. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Uključeno u slučaju greške. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---