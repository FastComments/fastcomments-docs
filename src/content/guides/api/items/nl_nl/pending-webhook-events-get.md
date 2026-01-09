[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

Deze route retourneert een lijst met openstaande webhook-events onder de parameter `pendingWebhookEvents`.

Deze API gebruikt paginering, geleverd door de `skip` parameter. PendingWebhookEvents worden geretourneerd in pagina's van `100`, gesorteerd op `createdAt` met de nieuwste eerst.

[inline-code-attrs-start title = 'Voorbeeld cURL voor PendingWebhookEvent'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PendingWebhookEvent-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Zoek naar gebeurtenissen waarvan de pogingentelling groter is dan het opgegeven aantal. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PendingWebhookEvent-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
    pendingWebhookEvents?: PendingWebhookEvent[]
}
[inline-code-end]