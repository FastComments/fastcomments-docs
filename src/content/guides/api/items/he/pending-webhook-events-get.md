[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

נתיב זה מחזיר רשימה של אירועי webhook ממתינים תחת פרמטר `pendingWebhookEvents`.

API זה משתמש בעימוד, המסופק על ידי פרמטר `skip`. PendingWebhookEvents מוחזרים בעמודים של `100`, מסודרים לפי `createdAt` החדש ביותר קודם.

[inline-code-attrs-start title = 'דוגמת cURL ל-PendingWebhookEvent'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetQueryParams {
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

[inline-code-attrs-start title = 'מבנה תגובת PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    pendingWebhookEvents?: PendingWebhookEvent[]
}
[inline-code-end]
