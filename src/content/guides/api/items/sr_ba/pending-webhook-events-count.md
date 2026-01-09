[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Ова рута враћа објекат који садржи број webhook догађаја који су на чекању у параметру `count`.

Можете филтрирати користећи исте параметре као и endpoint `/pending-webhook-events`

[inline-code-attrs-start title = 'cURL примјер за број PendingWebhookEvent'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за број PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Упит за догађаје чији је број покушаја већи од наведене вриједности. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за број PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    count?: number
}
[inline-code-end]