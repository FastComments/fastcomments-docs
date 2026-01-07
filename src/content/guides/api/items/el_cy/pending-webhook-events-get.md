[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει μια λίστα εκκρεμών γεγονότων webhook σε μια παράμετρο `pendingWebhookEvents`.

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από την παράμετρο `skip`. Τα PendingWebhookEvents επιστρέφονται σε σελίδες των `100`, ταξινομημένα κατά `createdAt` με τα πιο πρόσφατα πρώτα.

[inline-code-attrs-start title = 'PendingWebhookEvent cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Απάντησης PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
