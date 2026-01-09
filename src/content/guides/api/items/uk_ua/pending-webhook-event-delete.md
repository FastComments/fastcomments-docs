[api-resource-header-start name = 'PendingWebhookEvent'; route = 'DELETE /api/v1/pending-webhook-events/:id'; creditsCost = 1; api-resource-header-end]

Цей маршрут дозволяє видалити один `PendingWebhookEvent`.

Якщо потрібно виконати масове видалення, викличте GET API з пагінацією, а потім послідовно викликайте цей API.

[inline-code-attrs-start title = 'Приклад cURL (DELETE PendingWebhookEvent)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для видалення PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для видалення PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteResponse {
    status: 'success' | 'failed'
    /** Додається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Додається у разі невдачі. **/
    reason?: string
}
[inline-code-end]

---