[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

取得供稿中的貼文，最新的優先。分頁採用游標方式：傳遞您收到的最後一篇貼文的 `_id` 作為
`afterId` 以取得下一頁。

每返回十篇貼文消耗一點信用，最低消耗一點信用。

[inline-code-attrs-start title = 'FeedPost cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 在此貼文 ID 之後返回貼文。第一頁可省略此參數。 **/
    afterId?: string
    /** 預設為 10。最大值為 1000。 **/
    limit?: number
    /** 僅返回具有這些標籤的貼文。若有多個標籤，請重複此參數。 **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** 失敗時包含此欄位。 **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---