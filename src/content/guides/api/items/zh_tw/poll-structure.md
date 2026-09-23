A `Poll` 會附加在評論上，而不是作為獨立的物件。它會在建立評論時一起建立（請參考 `POST /api/v1/comments`），或之後使用 `PUT /api/v1/polls/:commentId` 加到既有評論上。

投票計數保存在投票本身上，因此讀取投票即可取得結果，無需自行加總。計算背後的個別投票則是 `PollVote` 物件。

每個選項都有在建立投票時產生的 `id`。此 id 用於投票、重新標記選項，以及在使用 `PUT` 更新投票時保留選項（及其投票），不論是新增或移除選項。這是唯一安全的引用方式——絕不要以列表中的位置來指代選項。

[inline-code-attrs-start title = '投票結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 當設定且時間已過去時，投票即關閉且不再接受投票。 **/
    closesAt?: string | null
    /** 0 匿名（預設），1 管理員與版主，2 所有人。未設定則視為匿名。 **/
    privacy?: 0 | 1 | 2 | null
    /** 當為 true 時，計數會對尚未投票的使用者隱藏。未設定則視為 false。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- 必須提供問題，且長度上限為 200 個字元。
- 投票必須有 2 到 10 個選項。
- 必須提供選項標籤，長度上限為 100 個字元，且在同一投票中必須唯一（不分大小寫）。
- `closesAt` 必須在投票建立時設定為未來的時間。若要立即關閉投票，可使用 `PATCH` 並將日期設為過去。

### Site Settings

投票遵循站台設定，您可以在「自訂小工具」下進行變更：

- 必須先啟用投票功能才能建立投票，否則 API 會回傳 `polls-disabled`。
- 投票可以限制為已登入的使用者；若僅以 `anonUserId` 送出投票，將被拒絕，回傳 `poll-login-required`。