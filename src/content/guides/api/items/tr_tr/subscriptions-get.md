[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Bu rota, `createdAt`'e göre sıralanmış, en yeni ilk olacak şekilde en fazla 30 `Subscription` nesnesi döndürür.

`userId` ile filtreleyebilirsiniz. SSO ile, kullanıcı kimliği `<tenant id>:<user id>` formatındadır.

[inline-code-attrs-start title = 'Kullanıcı İçin Abonelikler cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Abonelikler İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Kayıtları atlayarak sayfalama. **/
    skip?: number
    /** Kullanıcıya göre filtrele. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Abonelikler Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]