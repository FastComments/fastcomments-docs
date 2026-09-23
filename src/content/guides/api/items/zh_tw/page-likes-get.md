---
[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

返回頁面上的讚數，以及目前使用者是否已讚過。尚未存在的頁面會返回 `likeCount` 為 `0`。

[inline-code-attrs-start title = '頁面讚數 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI 編碼的您的 SSO 物件的 JSON。匿名使用者請省略。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: string
    /** 失敗時包含。 **/
    reason?: string
    likeCount: number
    /** 發出請求的使用者是否已讚過此頁面。 **/
    didLike: boolean
    /** 頁面上頂層評論的數量。 **/
    commentCount: number
    /** 用於訂閱此頁面即時更新的 ID。 **/
    urlIdWS: string
}
[inline-code-end]

---