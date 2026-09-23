[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

返回對頁面添加回應的使用者名稱，按字母順序排序。最多查詢 100 個回應，且不包括匿名使用者。

[inline-code-attrs-start title = '頁面回應使用者 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '頁面回應使用者 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** 回應 ID。 **/
    id: string
    /** URI 編碼的您的 SSO 物件 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面回應使用者 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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