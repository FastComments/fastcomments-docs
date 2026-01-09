[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

此路由會回傳最多 30 個 `Subscription` 物件，依 `createdAt` 排序，最新的在前。

你可以以 `userId` 過濾。使用 SSO 時，使用者 id 的格式為 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '取得使用者訂閱 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Subscriptions 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 透過跳過紀錄進行分頁。 **/
    skip?: number
    /** 以使用者過濾。 **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Subscriptions 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** 在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 在失敗時包含。 **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]