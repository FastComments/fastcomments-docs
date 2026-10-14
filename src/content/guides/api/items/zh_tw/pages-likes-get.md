[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

返回頁面上的讚數，並可選擇返回點讚的使用者，最近的優先顯示。這是 [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) 的伺服器端版本。

`includeUserInfo` 控制返回哪些使用者：

- `public` 會排除已將個人資料活動設為私密的使用者。
- `include-private` 會包含這些使用者。僅在列表不會向公眾顯示時使用。

匿名使用者永不會被包含，且最多會查詢 100 個讚。不存在的頁面會返回 `likeCount` 為 `0` 且 `users` 為空列表。

[inline-code-attrs-start title = '頁面讚數 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = '頁面讚數 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 實用提示

請記得對像 `urlId` 之類的值進行 URI 編碼。