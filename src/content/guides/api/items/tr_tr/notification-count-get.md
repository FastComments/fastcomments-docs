[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Bu rota kullanıcı kimliğine göre tek bir `NotificationCount` döndürür. SSO ile, kullanıcı kimliği `<tenant id>:<user id>` formatındadır.

Okunmamış bildirim yoksa bir `NotificationCount` olmayacaktır - bu nedenle 404 alırsınız.

Bu, `notifications/count`'den farklıdır: çok daha hızlıdır, ancak filtrelemeye izin vermez.

[inline-code-attrs-start title = 'NotificationCount ID ile cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]