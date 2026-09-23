[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

编辑投票而不影响其投票。可用于修正问题或选项中的拼写错误，关闭或重新打开投票，或更改谁可以看到投票者。

选项通过其 `id` 定位，`PATCH` 会重新标记您指定的选项。要添加、删除或重新排序选项，请将完整的选项列表发送至 `PUT /api/v1/polls/:commentId`：您发送的带有 id 的选项也会保留其投票。

每个字段都是可选的，但必须至少提供一个。

[inline-code-attrs-start title = '投票补丁 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = '立即关闭投票 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = '投票补丁请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** 重新标记现有选项。提供的每个 id 必须已经在投票中。 **/
    options?: { id: string, label: string }[] | null
    /** 过去的日期会立即关闭投票。null 会重新打开已关闭的投票。 **/
    closesAt?: string | null
    /** 0 匿名，1 管理员和版主，2 所有人。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票补丁响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** 失败时包含。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 其他说明

- 为投票中不存在的选项指定 id 会导致 `poll-invalid` 错误，而不是悄悄地不做任何操作。
- 标签在投票内必须保持唯一，包括您未更改的选项。
- 与创建投票不同，这里的 `closesAt` 可以是过去的时间——这就是立即关闭投票的方式。
- 投票的隐私设置在有投票后可以收紧，但不能放宽。
- 被锁定的评论无法更改其投票，并会返回 `locked` 错误。