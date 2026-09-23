A `Poll`（投票）附加在评论上，而不是作为独立对象。它随评论一起创建（参见 `POST /api/v1/comments`），或稍后使用 `PUT /api/v1/polls/:commentId` 添加到已有评论中。

投票计数保存在投票本身上，因此读取投票即可直接获得结果，无需自行汇总。这些计数背后的单个投票是 `PollVote` 对象。

每个选项都有一个在创建投票时生成的 `id`。该 id 用于投票、重新标记选项，以及在使用 `PUT` 更新投票时保留选项（及其投票），无论是添加还是删除选项。它是引用选项的唯一安全方式——不要使用选项在列表中的位置。

[inline-code-attrs-start title = '投票结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 设置后且时间在过去时，投票将关闭，不再接受投票。 **/
    closesAt?: string | null
    /** 0 匿名（默认），1 管理员和版主，2 所有人。缺失时表示匿名。 **/
    privacy?: 0 | 1 | 2 | null
    /** 为 true 时，计数对尚未投票的用户隐藏。缺失时表示 false。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- 必须提供问题，且最长 200 个字符。
- 投票必须包含 2 到 10 个选项。
- 必须提供选项标签，最长 100 个字符，并且在同一投票中必须唯一（不区分大小写）。
- 创建投票时，`closesAt` 必须是未来的时间。若要立即关闭投票，可使用 `PATCH` 并提供过去的日期。

### Site Settings

投票遵循站点配置，可在 “Customize Widget”（自定义小部件）下进行更改：

- 必须先启用投票功能，才能创建投票，否则 API 会返回 `polls-disabled`。
- 投票可以限制为已登录用户，此时仅包含 `anonUserId` 的投票请求会被拒绝，并返回 `poll-login-required`。

---