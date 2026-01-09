[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає до 30 об'єктів `Subscription`, відсортованих за `createdAt`, починаючи з найновіших.

Ви можете фільтрувати за `userId`. При SSO ідентифікатор користувача має формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Приклад cURL: підписки для користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту підписок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагінація шляхом пропуску записів. **/
    skip?: number
    /** Фільтр за користувачем. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для підписок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Додається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Додається у разі помилки. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---