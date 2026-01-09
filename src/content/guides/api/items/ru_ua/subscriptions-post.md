[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность создать `Subscription`. Обратите внимание, что у пользователя может быть только одна подписка на страницу, так как дополнительные подписки избыточны, и попытка
создать более одной подписки для одного и того же пользователя на одной и той же странице приведёт к ошибке.

Создание подписки приведёт к созданию объектов `Notification`, когда новый комментарий оставлен в корне подписанного `urlId` (когда `parentId` комментария равен `null`).

[inline-code-attrs-start title = 'Пример cURL-запроса для Subscription (POST)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Subscription POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Subscription POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]