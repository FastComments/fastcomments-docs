[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

此 API 端点提供为特定用户标记评论的功能。

Notes:

- 此调用必须始终在用户上下文中进行。该用户可以是 FastComments.com 用户、SSO 用户或租户用户。
- 如果设置了用于隐藏的标记阈值，评论在被标记达到定义次数后将会被实时自动隐藏。
- 在评论被自动取消批准（隐藏）之后，只有管理员或版主可以重新批准该评论。取消标记不会重新批准评论。

[inline-code-attrs-start title = '评论标记 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

对于匿名标记，我们必须指定一个 `anonUserId`。这可以是代表匿名会话的 ID，或随机 UUID。
这允许我们在用户未登录的情况下也支持对评论的标记和取消标记。这样，当使用相同的 `anonUserId` 获取评论时，该评论可以被标记为已标记。

[inline-code-attrs-start title = '匿名评论标记 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '评论标记请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '评论标记响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 失败时包含。 **/
    reason?: string
    /** 该评论是否因被标记过多而被取消批准（隐藏）？ **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---