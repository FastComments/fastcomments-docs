[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Diese Route gibt ein Objekt zurück, das die Anzahl der ausstehenden Webhook-Ereignisse unter einem `count`-Parameter enthält.

Sie können nach denselben Parametern wie beim `/pending-webhook-events`-Endpunkt filtern.

[inline-code-attrs-start title = 'PendingWebhookEvent Anzahl cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Anzahl Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Query for events with an attempt count greater than the specified number. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Anzahl Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    count?: number
}
[inline-code-end]
