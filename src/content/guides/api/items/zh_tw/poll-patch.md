[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

編輯投票而不影響其投票結果。可用於修正問題或選項中的錯字、關閉或
重新開啟投票，或變更誰可以看到投票者。

選項以其 `id` 為依據，`PATCH` 會重新標記您指定的選項。若要新增、移除或重新排序
選項，請將完整的選項清單傳送至 `PUT /api/v1/polls/:commentId`：您以 id 傳送的選項也會保留
其投票。

每個欄位皆為可選，但至少必須提供一個。

[inline-code-attrs-start title = '投票修補 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = '立即關閉投票 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = '投票修補請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** 重新標記現有選項。提供的每個 id 必須已存在於投票中。 **/
    options?: { id: string, label: string }[] | null
    /** 過去的日期會立即關閉投票。null 會重新開啟已關閉的投票。 **/
    closesAt?: string | null
    /** 0 匿名，1 管理員與版主，2 所有人。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票修補回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** 失敗時包含。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 其他說明

- 為投票未包含的選項 id 命名會導致 `poll-invalid` 錯誤，而不是靜默失敗。
- 標籤在投票內必須保持唯一，且需考慮未變更的選項。
- 與建立投票不同，此處的 `closesAt` 可以設定為過去的時間——這就是立即關閉投票的方式。
- 投票的隱私設定在有投票後只能收緊，無法放寬。
- 被鎖定的評論無法變更其投票，且會返回 `locked` 錯誤。

---