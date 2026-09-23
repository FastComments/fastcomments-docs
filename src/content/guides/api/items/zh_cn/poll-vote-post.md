[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

在投票上记录一次投票。

每个投票者对同一投票最多只能投一次。再次为同一投票者调用此接口会将其投票移动到新的选项，而不是添加第二个投票；如果投票的选项与其已选择的相同，则不会产生任何操作。

响应中包含投票对象，您可以直接获取更新后的计数，无需再次请求。

[inline-code-attrs-start title = 'PollVote 创建 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = '匿名 PollVote 创建 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** 必须提供 userId 或 anonUserId 其中之一。 **/
    userId?: string
    anonUserId?: string
    /** 最终用户的 IP，用于匿名速率限制。默认使用调用者的 IP。 **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** 失败时返回。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** 失败时返回。 **/
    reason?: string
    pollVote: PollVote
    /** 包含已更新计数的投票对象。 **/
    poll: CommentPoll
}
[inline-code-end]

### 匿名投票

使用 `anonUserId` 而不是 `userId` 来记录未登录用户的投票。该 ID 不需要在任何地方对应真实用户——它仅用于标识会话，从而避免同一人被计数多次。

匿名投票必须在您的站点上启用。如果投票仅限已登录用户，只有 `anonUserId` 的投票会因 `poll-login-required` 而失败。

匿名投票同样会对每个 IP、每个投票进行速率限制，以防止某人通过清除会话来刷票。请发送最终用户的 `ip`，这样限制会作用于用户本身，而不是您的服务器。

### 其他说明

- `userId` 必须是您站点上已存在的用户。对属于其他站点的用户的投票将被拒绝。
- 对已关闭的投票进行投票会因 `poll-closed` 而失败。
- 此 API 会更新投票的计数，并实时推送到已连接的小部件。