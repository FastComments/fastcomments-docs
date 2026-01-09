[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

此路由允许添加单个已授权的 `Vote`。投票可以是 `up` (+1) 或 `down` (-1)。

[inline-code-attrs-start title = 'Vote 创建 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '匿名 Vote 创建 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Vote 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### 创建匿名投票

可以通过在查询参数中设置 `anonUserId` 而不是 `userId` 来创建匿名投票。

该 id 不必与任何地方的用户对象对应（因此为匿名）。它只是会话的一个标识符
for the session, so you can fetch votes again in the same session, to check if a comment has
been voted.

如果您没有像 FastComments 那样的“匿名会话” - 您可以简单地
将其设置为随机 ID，例如 UUID（尽管我们更喜欢较短的标识符以节省空间）。

### 其他说明

- 该 API 遵守租户级设置。例如，如果您为某个页面禁用投票，并尝试通过 API 创建投票，则会以错误代码 `voting-disabled` 失败。
- 此 API 默认启用。
- 此 API 会更新相应 `Comment` 的 `votes`。

---