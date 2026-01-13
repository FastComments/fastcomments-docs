[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 30 объектов `Subscription`, отсортированных по `createdAt`, самые новые первыми.

Вы можете фильтровать по `userId`. При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL-запроса для подписок пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Subscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагинация через пропуск записей. **/
    skip?: number
    /** Фильтрация по пользователю. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Subscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включается при ошибке. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]