[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

从页面中移除当前用户的点赞。如果用户尚未点赞该页面，请求仍会成功，并返回代码 `not-liked`，且不会更改计数。

[inline-code-attrs-start title = '页面取消点赞 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '页面取消点赞请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** 您的 SSO 对象的 URI 编码 JSON。匿名用户请省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面取消点赞响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 当用户未点赞页面时为 'not-liked'。否则在失败时包含此字段。 **/
    code?: 'not-liked' | string
    /** 在失败时包含。 **/
    reason?: string
}
[inline-code-end]