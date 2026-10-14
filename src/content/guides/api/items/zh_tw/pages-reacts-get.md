[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

返回每個頁面上每種回應的計數，並可選擇返回回應的使用者，每筆回應以其 reaction id 為單位，按最新的先後順序排列。這是 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 的伺服器端版本。

`includeUserInfo` 控制返回哪些使用者：

- `public` 會排除已將個人資料活動設為私密的使用者。
- `include-private` 會包含他們。僅在列表不會向公眾顯示時使用此選項。

匿名使用者永不會被包含，且最多會查詢 100 筆回應。不存在的頁面不會返回 `counts`，且 `users` 列表為空。

[inline-code-attrs-start title = 'Page Reacts cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** 省略此項以僅取得計數。 **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失敗時包含。 **/
    reason?: string
    /** 每個 reaction id 的計數，例如 {"heart": 12, "laugh": 3}。當頁面沒有回應時不會設定。 **/
    counts?: Record<string, number>
    /** 每筆回應一個條目，最新的在前。僅在傳入 includeUserInfo 時設定。 **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** reaction id，例如 "heart"。 **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 實用提示

請記得對像 `urlId` 之類的值進行 URI 編碼。