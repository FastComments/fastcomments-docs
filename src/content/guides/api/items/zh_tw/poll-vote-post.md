[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

記錄在投票上的投票。

每位投票者對同一投票最多只能投一次。再次為相同投票者呼叫此 API 時，會將他們的投票移至新的選項，而不是新增第二票；若投票於已選擇的選項則不會產生任何變化。

回應中會包含投票本身，讓您在不需額外請求的情況下取得更新後的票數。

[inline-code-attrs-start title = 'PollVote 建立 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = '匿名 PollVote 建立 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 建立 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** 必須提供 userId 或 anonUserId 其中之一。 **/
    userId?: string
    anonUserId?: string
    /** 終端使用者的 IP，用於匿名速率限制。預設為呼叫者的 IP。 **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 建立 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** 失敗時會包含此欄位。 **/
    reason?: string
    pollVote: PollVote
    /** 包含已更新票數的投票。 **/
    poll: CommentPoll
}
[inline-code-end]

### 匿名投票

使用 `anonUserId` 取代 `userId` 以記錄未登入使用者的投票。此 ID 不必對應任何使用者——它僅用於識別會話，避免同一人被重複計算。

必須在您的站點啟用匿名投票。若投票僅限已登入使用者，僅提供 `anonUserId` 的投票會因 `poll-login-required` 而失敗。

匿名投票亦會依每個 IP、每個投票進行速率限制，以防止同一人透過清除會話來大量投票。請傳送終端使用者的 `ip`，讓限制套用於該使用者而非您的伺服器。

### 其他說明

- `userId` 必須是您站點上已存在的使用者。屬於其他站點的使用者投票會被拒絕。
- 在已關閉的投票上投票會因 `poll-closed` 而失敗。
- 此 API 會更新投票的票數，並即時推送至已連結的小工具。