---
[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

作为当前用户向页面添加一个反应。用户每个反应 ID 只能添加一次：再次添加时会成功返回代码 `already-reacted`，且计数不会改变。用户可以对同一页面添加多个不同的反应。

反应 ID 由您自行选择，最长可达 36 个字符。如果页面尚不存在，将会被创建。传递 `title` 可设置或更新页面的标题。

[inline-code-attrs-start title = '页面反应 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '页面反应请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面反应响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

---