[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Ανακτά δημοσιεύσεις σε ένα feed, από τις πιο πρόσφατες. Η σελιδοποίηση βασίζεται σε cursor: περάστε το `_id` της τελευταίας δημοσίευσης που λάβατε ως
`afterId` για να λάβετε την επόμενη σελίδα.

Κόστος ένα credit ανά δέκα επιστρεφόμενες δημοσιεύσεις, με ελάχιστο ένα credit.

[inline-code-attrs-start title = 'Παράδειγμα cURL FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Return posts after this post id. Omit for the first page. **/
    afterId?: string
    /** Defaults to 10. Maximum 1000. **/
    limit?: number
    /** Only return posts with these tags. Repeat the parameter for more than one tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Included on failure. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]