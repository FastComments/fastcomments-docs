A `PollVote` 是單個使用者對投票的回答。投票本身顯示的計數會與這些保持同步，因此只有在想知道 *誰* 投了哪個選項，而不是總計時才需要這些資料。

每位投票者在同一個投票中最多只能有一票。再次投票會將他們原本的投票移至新選項，而不是新增第二票，`updatedAt` 會記錄此變更的時間。

`voterId` 為投票者登入時的 `userId`，若未登入則為 `anonUserId`。

[inline-code-attrs-start title = 'PollVote 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** 投票者登入時的 userId，否則為 anonUserId。 **/
    voterId: string
    optionId: string
    createdAt: string
    /** 投票者最後一次將投票移至不同選項的時間。 **/
    updatedAt?: string
}
[inline-code-end]

### 隱私

投票的 `privacy` 設定對此 API 的作用方式與在評論小工具中的作用方式相同：

- **Anonymous**（預設）：沒有人能看到任何人的投票方式，因此投票內容無法被讀取。`GET /api/v1/poll-votes` 與 `GET /api/v1/poll-votes/:id` 會回傳 `poll-anonymous`。投票的計數仍可從 `GET /api/v1/polls/:commentId` 取得。
- **Admins and moderators**：您的 API 金鑰屬於您網站的管理員，因此可以讀取投票。
- **Everyone**：投票內容可被讀取。

投票一旦有投票紀錄，其隱私設定只能收緊，無法放寬。