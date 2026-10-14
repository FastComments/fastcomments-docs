[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

返回頁面上的讚數，以及目前使用者是否已讚過。尚未存在的頁面會回傳 `likeCount` 為 `0`。

傳入 `includeUserInfo=public` 以同時取得讚過此頁面的使用者，依最新時間排序。私人個人資料活動的使用者與匿名使用者會被排除，且最多查詢 100 個讚。此處僅接受 `public`；`include-private` 會回傳 `422`，且可透過帶有 API 金鑰的 [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) 取得。對於需要 SSO 的頁面，`includeUserInfo` 會對無法檢視頁面的使用者回傳 `403`。

[inline-code-attrs-start title = '頁面讚數 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** 您的 SSO 物件的 URI 編碼 JSON。匿名使用者請省略。 **/
    sso?: string
    /** 設定為 'public' 以包含讚過此頁面的使用者。 **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** 失敗時包含此欄位。 **/
    reason?: string
    likeCount: number
    /** 發出請求的使用者是否已讚過此頁面。 **/
    didLike: boolean
    /** 頁面上第一層評論的數量。 **/
    commentCount: number
    /** 用於訂閱此頁面即時更新的 ID。 **/
    urlIdWS: string
    /** 依最新時間排序。僅在傳入 includeUserInfo 時設定。 **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]