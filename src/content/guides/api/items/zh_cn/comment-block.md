[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

此 API 端点提供封锁撰写特定评论的用户的能力。它支持对由 FastComments.com 用户、SSO 用户和租户用户撰写的评论进行封锁。

它支持一个名为 `commentIdsToCheck` 的 body 参数，用于在执行此操作后检查客户端上是否有任何其他可能可见的评论应被封锁/解封。

Notes:

- 此调用必须始终在某个用户的上下文中进行。该用户可以是 FastComments.com 用户、SSO 用户或租户用户。
- 请求中的 `userId` 是正在*执行封锁*的用户。例如：`User A` 想封锁 `User B`。传入 `userId=User A` 以及 `User B` 所撰写的评论 ID。
- 完全匿名的评论（无用户 id、无电子邮件）无法被封锁，将返回错误。

[inline-code-attrs-start title = '评论封锁 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

对于匿名封锁，我们必须指定一个 `anonUserId`。这可以是表示匿名会话的 ID，或者是一个随机的 UUID。
这使我们能够即使在用户未登录的情况下也支持封锁评论，只要使用相同的 `anonUserId` 来获取评论。

[inline-code-attrs-start title = '匿名评论封锁 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '评论封锁 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = '评论封锁 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 在失败时包含。 **/
    reason?: string
    /** 如果定义了 commentIdsToCheck，该映射中为 true 的条目也会被封锁。 **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]