`Poll`（投票）是附加在评论上的，而不是一个独立的对象。它在创建评论时一并创建（参见 `POST /api/v1/comments`），或者稍后通过 `PUT /api/v1/polls/:commentId` 添加到已有评论上。

投票计数保存在投票对象本身上，因此读取投票时无需额外累加即可获得结果。这些计数背后的单个投票记录是 `PollVote` 对象。

每个选项都有一个在投票创建时生成的 `id`。该 id 用于投出选票、重新命名选项，以及在使用 `PUT` 更新投票（增删选项）时保留某个选项（及其选票）。这是引用选项的唯一安全方式——绝不要使用它在列表中的位置。

[inline-code-attrs-start title = 'Poll 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** 当已设置且时间已过，投票即关闭，不再接受选票。 **/
    closesAt?: string | null
    /** 0 为匿名（默认），1 为管理员和版主可见，2 为所有人可见。缺省表示匿名。 **/
    privacy?: 0 | 1 | 2 | null
    /** 为 true 时，尚未投票的用户无法看到计数。缺省表示 false。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### 限制

- 问题（question）为必填项，最多 200 个字符。
- 一个投票有 2 到 10 个选项。
- 选项标签为必填项，最多 100 个字符，且在同一投票内必须唯一（忽略大小写）。
- `closesAt` 在创建投票时必须是未来的时间。要立即关闭投票，请使用过去的日期对其进行 `PATCH` 操作。

### 站点设置

投票遵循你的站点配置，你可以在 Customize Widget（自定义小部件）下进行更改：

- 必须先启用投票功能才能创建投票，否则 API 会返回 `polls-disabled`。
- 投票可以限制为仅限已登录用户，在这种情况下，仅携带 `anonUserId` 发送的选票将被拒绝，并返回 `poll-login-required`。