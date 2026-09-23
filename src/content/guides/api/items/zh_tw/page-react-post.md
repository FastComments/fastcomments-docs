[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

以目前使用者的身份為頁面新增回應。使用者每個回應 id 只能新增一次：再次新增時會成功回傳代碼 `already-reacted`，且不會改變計數。使用者可以對同一頁面新增多個不同的回應。

回應 id 由您自行決定，長度上限為 36 個字元。如果頁面尚未存在，系統會自動建立。傳遞 `title` 以設定或更新頁面的標題。

[inline-code-attrs-start title = '頁面回應 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '頁面回應請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** 回應 ID，最多 36 個字元。 **/
    id: string
    /** 設定頁面的標題。 **/
    title?: string
    /** URI 編碼的您的 SSO 物件 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面回應回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 當使用者已經新增過此回應時返回 'already-reacted'。當 ID 超過 36 個字元時返回 'react-id-too-long'（HTTP 422）。其他失敗情況亦會包含此欄位。 **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** 失敗時包含此欄位。 **/
    reason?: string
}
[inline-code-end]