[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

此路由提供刪除單一 `Vote` 的功能。

[inline-code-attrs-start title = 'Vote 刪除 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote 刪除請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote 刪除回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** 失敗時包含此欄位。 **/
    reason?: string
}
[inline-code-end]

注意事項：

- 此 API 遵循租戶層級的設定。例如，如果您停用了某個頁面的投票功能，並嘗試透過 API 建立投票，則會失敗並回傳錯誤代碼 `voting-disabled`。
- 此 API 預設為即時生效。
- 此 API 會更新對應 `Comment` 的 `votes`。