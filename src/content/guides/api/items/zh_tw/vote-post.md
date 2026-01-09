[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

此路由提供新增單一已授權 `Vote` 的功能。投票可以是 `up` (+1) 或 `down` (-1)。

[inline-code-attrs-start title = '建立 Vote 的 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '建立匿名 Vote 的 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote 建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote 建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** 僅於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** 僅於失敗時包含。 **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### 建立匿名投票

可透過在查詢參數中設定 `anonUserId` 而非 `userId` 來建立匿名投票。

此 id 不必對應到任何地方的使用者物件（因此為匿名）。它只是一個識別符，
用於識別會話，讓你能在相同會話中再次取得投票，以檢查某則留言是否已被投票。

如果你沒有像 FastComments 那樣的「匿名會話」，你可以將其設為隨機 ID，例如 UUID（但我們偏好較短的識別符以節省空間）。

### 其他備註

- 此 API 遵從租戶層級的設定。例如，若你對特定頁面停用投票，並嘗試透過 API 建立投票，會以錯誤代碼 `voting-disabled` 失敗。
- 預設此 API 為上線狀態。
- 此 API 會更新對應 `Comment` 的 `votes`。