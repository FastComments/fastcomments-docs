[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

以当前用户的身份点赞页面。每个用户只能点赞一次：再次点赞会成功返回代码 `already-liked`，且不会改变计数。

如果页面尚未存在，将会被创建。传递 `title` 可设置或更新页面的标题。

[inline-code-attrs-start title = '页面点赞 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = '页面点赞请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** 设置页面的标题。 **/
    title?: string
    /** URI 编码的您的 SSO 对象的 JSON。匿名用户请省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面点赞响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 当用户已经点赞该页面时为 'already-liked'。否则在失败时包含此字段。 **/
    code?: 'already-liked' | string
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]