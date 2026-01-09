[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Цей маршрут повертає об'єкт, що містить кількість очікуваних webhook-подій у параметрі `count`.

Ви можете фільтрувати за тими ж параметрами, що й для `/pending-webhook-events` endpoint

[inline-code-attrs-start title = 'Приклад cURL запиту для PendingWebhookEvent Count'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту PendingWebhookEvent Count'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Пошук подій з кількістю спроб, більшою за вказане число. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PendingWebhookEvent Count'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включено у разі помилки. **/
    reason?: string
    count?: number
}
[inline-code-end]