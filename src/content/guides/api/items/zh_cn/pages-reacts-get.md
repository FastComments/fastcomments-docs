[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

返回页面上每种反应的计数，并可选返回已反应的用户，每条记录对应一个反应及其 reaction id，按最近的先后顺序排列。这是 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 的服务器端实现。

`includeUserInfo` 控制返回哪些用户：

- `public` 排除已将个人资料活动设为私密的用户。
- `include-private` 包含这些用户。仅在列表不向公众展示时使用。

匿名用户永不包含在内，最多会查询 100 条反应。不存在的页面返回没有 `counts`，且 `users` 列表为空。

[inline-code-attrs-start title = '页面反应 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = '页面反应 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** 省略此项仅获取计数。 **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失败时包含。 **/
    reason?: string
    /** 每个 reaction id 的计数，例如 {"heart": 12, "laugh": 3}。当页面没有任何反应时不设置此字段。 **/
    counts?: Record<string, number>
    /** 每个反应一条记录，按最近的先后顺序。仅在传入 includeUserInfo 时设置。 **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** reaction id，例如 "heart"。 **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 有用提示

请记得对类似 `urlId` 的值进行 URI 编码。

---