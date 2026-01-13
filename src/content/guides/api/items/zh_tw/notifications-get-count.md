[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

此路由會回傳一個物件，包含在 `count` 參數下的通知數量。

它比 `/notification-count/` 慢且花費雙倍點數，但允許在更多維度上過濾。

您可以使用與 `/notifications` 端點相同的參數進行過濾，例如 `userId`。使用 SSO 時，使用者 ID 的格式為 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '使用者未讀通知數量 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '特定頁面使用者未讀通知數量 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '通知數量請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 依使用者過濾。 **/
    userId?: string
    /** 依 urlId 過濾。 **/
    urlId?: string
    /** 依來源留言過濾。 **/
    fromCommentId?: string
    /** 依已讀/未讀 過濾。 **/
    viewed?: 'true' | 'false'
    /** 依類型過濾。 **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '通知數量回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失敗時包含。 **/
    reason?: string
    count?: number
}
[inline-code-end]