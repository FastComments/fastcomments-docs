[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

返回页面的点赞数量，并可选返回点赞用户，按最近的先后顺序。这是 [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) 的服务器端版本。

`includeUserInfo` 控制返回哪些用户：

- `public` 排除已将个人资料活动设为私密的用户。
- `include-private` 包含这些用户。仅在列表不向公众展示时使用。

匿名用户永不被包含，且最多查询 100 条点赞。不存在的页面返回 `likeCount` 为 `0`，并且 `users` 列表为空。

[inline-code-attrs-start title = '页面点赞 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = '页面点赞请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = '页面点赞响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### 有用提示

请记得对类似 `urlId` 的值进行 URI 编码。

---