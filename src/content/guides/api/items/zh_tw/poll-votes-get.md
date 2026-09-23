[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

列出單一投票的個別票項，依投票時間由舊到新排序。每 100 票返回消耗 1 點信用。

投票屬於某則評論，因此一次只能讀取一個投票的票項，且必須提供 `commentId`。可進一步使用 `voterId` 來檢查某位使用者的投票，或使用 `optionId` 列出所有選擇特定選項的使用者。

每次呼叫最多返回 1000 票。使用 `skip` 參數可分頁取得更多。

會遵守投票的 `privacy` 設定：匿名投票的票項無法被讀取，請求會以 `poll-anonymous` 錯誤失敗。詳情請參閱 `PollVote` 結構。

[inline-code-attrs-start title = 'PollVotes 取得 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes 取得 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes 取得 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### 依選項計算票數

您不需要自行加總以取得結果——投票本身已包含計數。請改以 `GET /api/v1/polls/:commentId` 讀取投票，當您需要知道誰投了票時再使用此 API。

### 每頁的所有投票

沒有整頁的投票列表。若要對整頁進行報告，請使用 `GET /api/v1/comments` 取得該頁的評論，該回應會返回每則評論的投票及其計數，之後再針對您關心的投票讀取票項。