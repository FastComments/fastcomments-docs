[api-resource-header-start name = 'Subscription'; route = 'DELETE /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Bu rota bir `Subscription` nesnesini id ile siler.

[inline-code-attrs-start title = 'Abonelik Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/subscriptions/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Abonelik Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Abonelik Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]