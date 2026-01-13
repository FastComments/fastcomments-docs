[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

此路由用於建立單一 SSO 使用者。

嘗試建立兩個具有相同 ID 的使用者會導致錯誤。

[inline-code-attrs-start title = 'SSOUser 建立 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

在此範例中我們為存取控制指定了 `groupIds`，但這是可選的。

[inline-code-attrs-start title = 'SSOUser 建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 失敗時會包含。 **/
    reason?: string
    user?: SSOUser; // 成功時會回傳建立的使用者。
}
[inline-code-end]

#### 整合注意事項

透過 API 傳遞的資料可以透過傳遞不同的 SSO User HMAC payload 來覆寫。例如，如果
您透過 API 設定了一個 username，但在頁面載入時透過 SSO 流程傳遞了不同的 username，我們會自動更新
他們的 username。

除非您明確指定或將其設為 null（不是 undefined），否則我們不會在此流程中更新使用者參數。