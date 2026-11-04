`Poll` 是附加在留言上的，而不是獨立的物件。它會隨留言一起建立（見 `POST /api/v1/comments`），或稍後透過 `PUT /api/v1/polls/:commentId` 加入到現有的留言。

投票計數保存在投票本身，因此讀取投票時即可直接取得結果，無需自行加總。這些計數背後的個別投票則是 `PollVote` 物件。

每個選項都有一個在投票建立時產生的 `id`。這個 id 用於投票、重新命名選項，以及在您以 `PUT` 更新投票（新增或移除選項）時保留某個選項（及其投票）。這是引用選項唯一安全的方式——絕不要使用它在清單中的位置。

[inline-code-attrs-start title = 'Poll 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 當有設定且時間已過，投票即關閉，不再接受投票。 **/
    closesAt?: string | null
    /** 0 為匿名（預設），1 為管理員與版主，2 為所有人。未提供表示匿名。 **/
    privacy?: 0 | 1 | 2 | null
    /** 當為 true 時，尚未投票者無法看到計數。未提供表示 false。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### 限制

- 問題為必填，最多 200 個字元。
- 一個投票需有 2 到 10 個選項。
- 選項標籤為必填，最多 100 個字元，且在同一投票內必須唯一（不區分大小寫）。
- `closesAt` 在建立投票時必須是未來的時間。若要立即關閉投票，請以過去的日期進行 `PATCH`。

### 網站設定

投票遵循您的網站設定，您可以在 Customize Widget 下進行更改：

- 必須先啟用投票功能才能建立投票，否則 API 會回應 `polls-disabled`。
- 投票可以限制為已登入的使用者，在此情況下，僅帶有 `anonUserId` 的投票會被拒絕並回應 `poll-login-required`。