[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

此路由提供添加单个授权 `Vote` 的能力。投票可以是 `up`（+1）或 `down`（-1）。

[inline-code-attrs-start title = '创建投票 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '创建匿名投票 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '创建投票请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = '创建投票响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** 失败时包含。 **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### 创建匿名投票

可以通过在查询参数中设置 `anonUserId`（而不是 `userId`）来创建匿名投票。

此 ID 不必对应任何地方的用户对象（因此是匿名的）。它只是会话的标识符，以便您可以在同一会话中再次获取投票，以检查某条评论是否已被投票。

如果您没有像 FastComments 那样的“匿名会话”概念，您可以简单地将其设置为一个随机 ID，例如 UUID（不过我们更希望使用较短的标识符以节省空间）。

### 其他说明

- 此 API 遵循租户级别的设置。例如，如果您禁用了某个页面的投票功能，并尝试通过 API 创建投票，则会失败并返回错误代码 `voting-disabled`。
- 此 API 默认是实时的。
- 此 API 会更新相应 `Comment` 的 `votes`。