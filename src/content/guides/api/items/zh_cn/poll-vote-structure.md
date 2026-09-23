A `PollVote` 是一个人对投票的答案。投票本身显示的计数会与这些保持同步，因此只有在想知道*谁*投了什么，而不是总计时才需要这些。

每个投票人在同一投票中最多只能投一次。再次投票会将其已有的投票移动到新选项，而不是添加第二个投票，`updatedAt` 记录了发生的时间。

`voterId` 是投票人登录时的 `userId`，否则为 `anonUserId`。

[inline-code-attrs-start title = 'PollVote 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** 投票人登录时的 userId，否则为 anonUserId。 **/
    voterId: string
    optionId: string
    createdAt: string
    /** 投票人上次将投票移动到不同选项的时间。 **/
    updatedAt?: string
}
[inline-code-end]

### 隐私

投票的 `privacy` 设置对该 API 的作用方式与在评论小部件中的作用方式相同：

- **Anonymous**（默认）：没有人能看到任何人的投票方式，因此投票内容不可读取。`GET /api/v1/poll-votes` 和 `GET /api/v1/poll-votes/:id` 会返回 `poll-anonymous`。投票的计数仍可通过 `GET /api/v1/polls/:commentId` 获取。
- **Admins and moderators**：你的 API 密钥属于站点的管理员，因此可以读取投票。
- **Everyone**：投票可以被读取。

投票一旦产生，投票的隐私设置可以收紧但不能放宽。