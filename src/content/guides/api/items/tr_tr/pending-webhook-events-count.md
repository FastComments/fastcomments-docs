[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

Bu rota, bekleyen webhook olaylarının sayısını `count` parametresi altında içeren bir nesne döndürür.

Aynı parametrelere göre `/pending-webhook-events` uç noktasıyla filtreleyebilirsiniz.

[inline-code-attrs-start title = 'PendingWebhookEvent Sayısı cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Sayısı İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** Belirtilen sayıdan daha yüksek deneme sayısına sahip olayları sorgular. **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Sayısı Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    count?: number
}
[inline-code-end]

---