[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

此 API 端点提供解除对撰写给定评论的用户的封禁的功能。它支持对由 FastComments.com 用户、SSO 用户和租户用户撰写的评论进行解除封禁。

它支持一个 `commentIdsToCheck` 请求体参数，用于在执行此操作后检查客户端上其他可能可见的评论是否应被封禁/解封。

注意：

- 此调用必须始终在用户上下文中进行。该用户可以是 FastComments.com 用户、SSO 用户或租户用户。
- 请求中的 `userId` 是正在执行解除封禁操作的用户。例如：`User A` 想要解除对 `User B` 的封禁。传入 `userId=User A` 以及 `User B` 所写的评论 id。
- 完全匿名的评论（没有用户 id、没有电子邮件）无法被封禁，会返回错误。

[inline-code-attrs-start title = '评论 解除封禁 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '匿名评论 解除封禁 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '评论 解除封禁 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = '评论 解除封禁 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 失败时包含。 **/
    reason?: string
    /** 如果定义了 commentIdsToCheck，则此映射中值为 true 的条目仍然被封禁。若为 false，则您可能需要对用户取消隐藏这些评论，以免他们需要刷新页面。 **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---