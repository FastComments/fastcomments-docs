[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 30 объектов `Subscription`, отсортированных по `createdAt`, в порядке от новых к старым.

Вы можете фильтровать по `userId`. При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL-запроса подписок для пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса подписок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пропустить указанное количество записей для пагинации. **/
    skip?: number
    /** Фильтровать по пользователю. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа подписок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Включается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включается при неудаче. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---