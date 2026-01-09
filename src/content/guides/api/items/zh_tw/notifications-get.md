[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

此路由會回傳最多 30 個 `Notification` 物件，依 `createdAt` 排序，最新的在前。

你可以以 `userId` 過濾。使用 SSO 時，使用者 id 的格式為 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '使用者未讀通知 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '通知請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 透過跳過記錄進行分頁。 **/
    skip?: number
    /** 以使用者過濾。 **/
    userId?: string
    /** 以 urlId 過濾。 **/
    urlId?: string
    /** 以來源評論過濾。 **/
    fromCommentId?: string
    /** 以已讀/未讀 過濾。 **/
    viewed?: 'true' | 'false'
    /** 以類型過濾。 **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '通知回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失敗時包含。 **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]