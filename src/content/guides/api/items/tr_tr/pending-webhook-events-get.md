[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

Bu rota, `pendingWebhookEvents` parametresi altında bekleyen webhook olaylarının bir listesini döndürür.

Bu API, `skip` parametresiyle sağlanan sayfalama kullanır. PendingWebhookEvents `100` öğe halinde sayfalanmış şekilde döndürülür, `createdAt` alanına göre en yeniler ilk sırada olacak şekilde sıralanır.

[inline-code-attrs-start title = 'PendingWebhookEvent cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Deneme sayısı belirtilen sayıdan daha yüksek olan olayları sorgular. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
    pendingWebhookEvents?: PendingWebhookEvent[]
}
[inline-code-end]

---