[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

从页面中移除当前用户的一个反应。如果用户未添加该反应，请求将以代码 `no-react` 成功返回，并且计数不会改变。

[inline-code-attrs-start title = '页面反应删除 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '页面反应删除请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** 反应 ID。 **/
    id: string
    /** URI 编码的您的 SSO 对象的 JSON。匿名用户请省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应删除响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' 当用户未添加此反应时。否则在失败时包含。 **/
    code?: 'no-react' | string
    /** 在失败时包含。 **/
    reason?: string
}
[inline-code-end]