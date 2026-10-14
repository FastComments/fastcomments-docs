[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

返回页面上每个反应的计数，以及当前用户已添加的反应。

传入 `includeUserInfo=public` 还可以获取已反应的用户，每个 react 对应一条记录，按 reaction id 排序，最近的在前。私有资料活动的用户和匿名用户会被排除，最多查询 100 条 react。此处仅接受 `public`；`include-private` 会返回 `422`，并可通过带 API key 的 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) 使用。在需要 SSO 的页面上，`includeUserInfo` 对无法查看页面的用户会返回 `403`。

[inline-code-attrs-start title = '页面反应 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '页面反应请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI 编码的您的 SSO 对象的 JSON。匿名用户请省略。 **/
    sso?: string
    /** 设置为 'public' 以包含已反应的用户。 **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** 在失败时包含。 **/
    reason?: string
    /** 每个 reaction id 的计数，例如 {"heart": 12, "laugh": 3}。当页面没有反应时不设置。 **/
    counts?: Record<string, number>
    /** 发起请求的用户已添加的 reaction id。若未添加则不设置。 **/
    reactedIds?: string[]
    /** 每个 react 一条记录，最近的在前。仅在传入 includeUserInfo 时设置。 **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** reaction id，例如 "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]