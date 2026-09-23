[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

將投票附加到現有的評論，或設定其已存在投票的完整狀態。

請求主體是完整的投票，您傳送的選項將成為投票的選項，依序排列。每個選項會依其 `id` 進行匹配：

- 傳送帶有現有選項 `id` 的選項會保留該選項及其投票。其標籤與位置會更新為您傳送的內容。
- 未帶 `id` 的選項會被新增，且沒有投票。
- 若省略現有的選項，該選項會被移除，連同其投票一起刪除。`totalVotes` 會相應減少相同數量。

因此，要新增選項，請傳送帶有其 id 的現有選項以及不帶 id 的新選項。要移除選項，則在清單中省略該選項。選項的 id 可在 `GET /api/v1/polls/:commentId` 回傳的投票中取得。

若完全不傳送任何 id，則會取代所有選項並刪除投票中已存在的所有投票。若投票已有投票，必須加上 `replaceVotes=true`，否則 API 會回應 `replace-votes-required`。

其他欄位也會被取代：若省略 `closesAt`、`privacy` 或 `requireVoteToSeeResults`，則會重設為預設值。若只想變更單一欄位而保留其他不變，請使用 `PATCH /api/v1/polls/:commentId`。

[inline-code-attrs-start title = '投票 PUT cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '投票 PUT 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 當投票已有投票時，需要此參數以保留所有現有選項 id，因為不提供會刪除全部。 **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** 現有選項的 id，用於保留該選項及其投票。省略則新增選項。 **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** 完整且有順序的列表。省略的現有選項會被移除，連同其投票一起刪除。 **/
    options: PollPutOption[]
    /** 當評論尚未有投票時，必須是未來的時間。若投票保持開放，則可省略。 **/
    closesAt?: string | null
    /** 0 匿名（預設），1 管理員與版主，2 所有人。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票 PUT 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** 失敗時會包含此欄位。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 其他說明

- 若 `id` 不在投票中，或同一個 `id` 重複提供，會失敗並回傳 `poll-invalid`。尚未有投票的評論沒有任何選項 id，因此傳送給它的每個選項都必須省略 `id`。
- 投票的隱私設定在有投票後只能收緊，無法放寬。
- 此 API 會遵循站台設定。若站台或頁面未啟用投票，則會失敗並回傳 `polls-disabled`。
- 被鎖定的評論無法變更其投票，會失敗並回傳 `locked`。
- 已連結的元件會即時更新，觀眾可在不重新載入的情況下看到新的投票。

---