[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

返回页面上每个反应的计数，以及当前用户已添加的反应。

[inline-code-attrs-start title = '页面反应 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '页面反应 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** 您的 SSO 对象的 URI 编码 JSON。匿名用户请省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: string
    /** 失败时包含。 **/
    reason?: string
    /** 每个反应 ID 的计数，例如 {"heart": 12, "laugh": 3}。当页面没有反应时不设置。 **/
    counts?: Record<string, number>
    /** 发起请求的用户已添加的反应 ID。若未添加则不设置。 **/
    reactedIds?: string[]
}
[inline-code-end]