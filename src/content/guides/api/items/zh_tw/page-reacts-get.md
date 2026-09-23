[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

返回頁面上每個回應的計數，以及當前使用者已加入的回應。

[inline-code-attrs-start title = 'Page Reacts cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI 編碼的您的 SSO 物件的 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: string
    /** 失敗時包含。 **/
    reason?: string
    /** 每個回應 ID 的計數，例如 {"heart": 12, "laugh": 3}。當頁面沒有回應時不會設定。 **/
    counts?: Record<string, number>
    /** 發出請求的使用者已加入的回應 ID。若未加入任何回應則不會設定。 **/
    reactedIds?: string[]
}
[inline-code-end]