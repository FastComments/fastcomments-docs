[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

從頁面中移除目前使用者的一個回應。如果使用者尚未加入該回應，請求仍會成功，回傳代碼 `no-react`，且不會改變計數。

[inline-code-attrs-start title = '頁面回應刪除 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '頁面回應刪除請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** 回應 ID。 **/
    id: string
    /** 您的 SSO 物件的 URI 編碼 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面回應刪除回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 當使用者未加入此回應時為 'no-react'。其他情況於失敗時包含此代碼。 **/
    code?: 'no-react' | string
    /** 失敗時包含。 **/
    reason?: string
}
[inline-code-end]