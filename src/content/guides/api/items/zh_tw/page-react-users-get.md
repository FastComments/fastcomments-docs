[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

返回對頁面添加了回應的使用者名稱，按字母順序排序。最多查詢 100 個回應，匿名使用者以及具有私人個人資料活動的使用者不會被包含。對於需要 SSO 的頁面，無法檢視該頁面的使用者會收到 `403`，錯誤代碼為 `sso-required` 或 `page-forbidden`。

若也想取得頭像與使用者 ID，請使用 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 並加上 `includeUserInfo=public`。

[inline-code-attrs-start title = '頁面反應使用者 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '頁面反應使用者 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** 反應 ID。 **/
    id: string
    /** URI 編碼的 JSON SSO 物件。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面反應使用者 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: string
    /** 失敗時包含。 **/
    reason?: string
    userNames: string[]
}
[inline-code-end]