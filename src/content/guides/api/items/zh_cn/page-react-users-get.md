[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

返回对页面添加了反应的用户的名称，按字母顺序排序。最多查找 100 条反应，匿名用户不包括在内。

[inline-code-attrs-start title = '页面反应用户 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '页面反应用户 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** 反应 ID。 **/
    id: string
    /** 您的 SSO 对象的 URI 编码 JSON。匿名用户请省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应用户 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: string
    /** 失败时包含。 **/
    reason?: string
    userNames: string[]
}
[inline-code-end]