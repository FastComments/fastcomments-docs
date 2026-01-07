[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Този маршрут връща до 30 обекта `Subscription`, сортирани по `createdAt`, от най-новите.

Можете да филтрирате по `userId`. При SSO потребителският id е във формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример за абонаменти на потребител с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за Subscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginate by skipping records. **/
    skip?: number
    /** Filter by user. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за Subscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]
