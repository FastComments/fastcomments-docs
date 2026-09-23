[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

列出单个投票，按时间顺序（最早的在前）对应于某个投票的计数。每返回 100 条投票消耗 1 个积分。

投票属于某条评论，因此一次只能读取一个投票的投票记录，并且需要提供 `commentId`。可使用 `voterId` 进一步筛选以查看某个人的投票情况，或使用 `optionId` 列出选择了特定选项的所有投票者。

每次调用最多返回 1000 条投票记录。可使用 `skip` 参数进行分页以获取更多。

投票的 `privacy` 设置会被遵守：匿名投票的投票记录无法读取，请求会返回 `poll-anonymous` 错误。详情请参阅 `PollVote` 结构。

[inline-code-attrs-start title = 'PollVotes 获取 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes 获取请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes 获取响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### 按选项统计投票

您无需自行汇总投票结果——投票本身已经包含计数。请使用 `GET /api/v1/polls/:commentId` 读取投票信息，仅在需要了解投票者时才使用此接口。

### 页面上的每个投票

没有针对整页的投票列表。若要对整页进行报告，请使用 `GET /api/v1/comments` 获取该页的评论，该接口会返回每条评论的投票及其计数，然后再读取您关心的投票的投票记录。

---