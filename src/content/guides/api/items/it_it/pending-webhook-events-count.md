[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Questa route restituisce un oggetto contenente il numero di eventi webhook in sospeso sotto il parametro `count`.

Puoi filtrare con gli stessi parametri dell'endpoint `/pending-webhook-events`

[inline-code-attrs-start title = 'Esempio cURL Conteggio PendingWebhookEvent'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta Conteggio PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Ricerca eventi con un numero di tentativi maggiore del valore specificato. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Conteggio PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    count?: number
}
[inline-code-end]

---