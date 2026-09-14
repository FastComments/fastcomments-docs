[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

获取 feed 中的帖子，按最新排序。分页基于游标：将您收到的最后一篇帖子的 `_id` 作为 `afterId` 传递，以获取下一页。

每返回十篇帖子消耗一个积分，最低消耗一个积分。

[inline-code-attrs-start title = 'FeedPost cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 返回此帖子 ID 之后的帖子。首次页面请省略。 **/
    afterId?: string
    /** 默认值为 10。最大值为 1000。 **/
    limit?: number
    /** 仅返回带有这些标签的帖子。若有多个标签，请重复该参数。 **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** 失败时包含此字段。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** 失败时包含此字段。 **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]