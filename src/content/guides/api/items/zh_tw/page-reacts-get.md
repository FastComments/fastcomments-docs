[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

返回頁面上每個回應的計數，以及目前使用者已添加的回應。

傳入 `includeUserInfo=public` 以同時取得已回應的使用者，每個回應會以一筆資料列出其回應 ID，且依最新的排序。私人個人資料活動的使用者與匿名使用者會被排除，最多會查詢 100 筆回應。此處僅接受 `public`；`include-private` 會回傳 `422`，且可透過帶有 API 金鑰的 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) 取得。對於需要 SSO 的頁面，`includeUserInfo` 會對無法檢視頁面的使用者回傳 `403`。

[inline-code-attrs-start title = '頁面回應 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '頁面回應 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** 您的 SSO 物件的 URI 編碼 JSON。匿名使用者請省略。 **/
    sso?: string
    /** 設定為 'public' 以包含已回應的使用者。 **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '頁面回應 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含此欄位。 **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** 失敗時會包含此欄位。 **/
    reason?: string
    /** 每個回應 ID 的計數，例如 {"heart": 12, "laugh": 3}。若頁面沒有回應則不設定此欄位。 **/
    counts?: Record<string, number>
    /** 發出請求的使用者已添加的回應 ID。若未添加任何回應則不設定此欄位。 **/
    reactedIds?: string[]
    /** 每個回應一筆資料，依最新排序。僅在傳入 includeUserInfo 時設定。 **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** 回應 ID，例如 "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]