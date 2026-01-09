[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ця кінцева точка API надає можливість створити `Subscription`. Зауважте, що користувач може мати лише одну підписку на сторінку, оскільки більше є надлишковим, і спроба створити більше ніж одну підписку для того самого користувача для тієї самої сторінки призведе до помилки.

Створення підписки призведе до створення об'єктів `Notification`, коли на корені підписаного `urlId` буде залишено новий коментар (коли `parentId` коментаря дорівнює `null`).

[inline-code-attrs-start title = 'Приклад cURL-запиту для створення підписки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура POST-запиту підписки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді POST-запиту підписки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Включається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Включається у разі невдачі. **/
    reason?: string
}
[inline-code-end]