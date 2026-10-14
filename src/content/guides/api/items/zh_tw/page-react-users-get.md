[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

返回對頁面添加了回應的使用者名稱，按字母順序排序。最多查詢 100 個回應，匿名使用者不會被包含。在需要 SSO 的頁面上，無法查看頁面的使用者會收到代碼為 `sso-required` 或 `page-forbidden` 的 `403`。

若也想取得頭像與使用者 ID，請使用 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 並加上 `includeUserInfo=public`。

[inline-code-attrs-start title = '頁面回應使用者 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '頁面回應使用者 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面回應使用者 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]