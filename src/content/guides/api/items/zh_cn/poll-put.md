[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

将投票附加到已有评论，或设置其已有投票的完整状态。

请求体是完整的投票，您发送的选项将按顺序成为投票的选项。每个选项通过其 `id` 进行匹配：

- 发送了已有选项 `id` 的选项会保留该选项及其投票。其标签和位置会更新为您发送的内容。
- 未提供 `id` 的选项会被添加，且没有投票。
- 您省略的已有选项会被移除，连同其投票一起。`totalVotes` 会相应减少相同数量。

因此，要添加选项，请发送包含其 id 的当前选项并再加上一个没有 id 的新选项。要删除选项，只需在列表中省略它。选项 id 可在 `GET /api/v1/polls/:commentId` 返回的投票中获取。

如果完全不发送 id，则会替换所有选项并删除投票中已投的所有票。如果投票已有投票，则需要 `replaceVotes=true`，否则 API 会返回 `replace-votes-required`。

其他字段也会被替换：省略 `closesAt`、`privacy` 或 `requireVoteToSeeResults` 会将其重置为默认值。若只想更改单个字段而保持其他不变，请使用 `PATCH /api/v1/polls/:commentId`。

[inline-code-attrs-start title = '投票 PUT cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = '投票 PUT 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 当投票已有投票时，需要此字段以保留所有现有选项 id，因为不保留会删除它们全部。 **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** 已有选项的 id，用于保留该选项及其投票。省略则添加新选项。 **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** 完整的有序列表。省略的已有选项会连同其投票一起被移除。 **/
    options: PollPutOption[]
    /** 当评论尚未有投票时，必须是未来的时间。若投票保持开放则省略此字段。 **/
    closesAt?: string | null
    /** 0 匿名（默认），1 管理员和版主，2 所有人。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票 PUT 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** 失败时包含。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 其他说明

- `id` 不在投票中，或同一 `id` 重复出现，都会导致 `poll-invalid` 错误。没有投票的评论尚未拥有选项 id，因此发送的每个选项都必须省略 `id`。
- 投票的隐私设置在已有投票后可以收紧，但不能放宽。
- 此 API 遵循站点设置。如果站点或页面未启用投票，则会返回 `polls-disabled` 错误。
- 被锁定的评论无法更改其投票，会返回 `locked` 错误。
- 已连接的小部件会实时更新，观众无需刷新即可看到新的投票。

---