[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

此路由创建一个单独的 `FeedPost`。每个帖子都有作者，因此 `fromUserId` 为必填项，且必须是账户中已存在的 FastComments 或 SSO 用户的 ID。

[inline-code-attrs-start title = 'FeedPost 创建 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** 将帖子推送到当前在浏览器中打开的 feed。默认值为 false。 **/
    isLive?: boolean
    /** 在保存之前通过垃圾信息引擎处理帖子。默认值为 false。 **/
    doSpamCheck?: boolean
    /** 跳过作为 doSpamCheck 一部分运行的重复内容检查。默认值为 false。 **/
    skipDupCheck?: boolean
    /** 最多 256 个字符。回显给实时监听者，以便客户端可以忽略自己的广播。 **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** 必填。FastComments 或 SSO 用户的 ID。 **/
    fromUserId: string
    title?: string
    /** HTML。保存时会进行清理。 **/
    contentHTML?: string
    /** 覆盖从用户获取的显示名称。 **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** 失败时包含。 **/
    reason?: string
    feedPost?: FeedPost; // 成功时返回完整创建的帖子。
}
[inline-code-end]