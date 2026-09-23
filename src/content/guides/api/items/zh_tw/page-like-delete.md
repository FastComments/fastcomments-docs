[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

移除目前使用者對頁面的讚。如果使用者尚未對該頁面按讚，請求仍會成功，回傳代碼 `not-liked`，且不會改變計數。

[inline-code-attrs-start title = '頁面取消讚 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '頁面取消讚請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI 編碼的您的 SSO 物件的 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面取消讚回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 當使用者未對頁面按讚時為 'not-liked'。其他情況在失敗時會包含此欄位。 **/
    code?: 'not-liked' | string
    /** 失敗時會包含。 **/
    reason?: string
}
[inline-code-end]

---