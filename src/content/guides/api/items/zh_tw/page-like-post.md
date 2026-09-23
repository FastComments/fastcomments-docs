[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

以當前使用者的身份為頁面點讚。每位使用者只能對同一頁面點讚一次：再次點讚會成功回傳代碼 `already-liked`，且不會改變計數。

如果頁面尚未存在，系統會自動建立。傳遞 `title` 以設定或更新頁面的標題。

[inline-code-attrs-start title = '頁面點讚 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = '頁面點讚請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面點讚回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' when the user had already liked the page. Otherwise included on failure. **/
    code?: 'already-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]