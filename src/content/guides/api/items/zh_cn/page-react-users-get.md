[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

返回对页面添加了反应的用户的名称，按字母顺序排序。最多查找 100 条反应，匿名用户和拥有私人资料活动的用户不包括在内。对于需要 SSO 的页面，无法查看页面的用户会收到状态码 `403`，代码为 `sso-required` 或 `page-forbidden`。

若要同时获取头像和用户 ID，请使用 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 并添加 `includeUserInfo=public`。

[inline-code-attrs-start title = '页面反应用户 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '页面反应用户 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应用户 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]