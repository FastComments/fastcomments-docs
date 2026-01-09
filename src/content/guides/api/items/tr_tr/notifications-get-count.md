[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Bu rota, bildirim sayısını `count` parametresi altında içeren bir nesne döndürür.

Bu, `/notification-count/`'den daha yavaştır ve kredi maliyeti iki katıdır, ancak daha fazla boyuta göre filtrelemeye izin verir.

`/notifications` uç noktasındaki `userId` gibi aynı parametrelere göre filtreleyebilirsiniz. SSO ile kullanıcı kimliği `<tenant id>:<user id>` formatındadır.

[inline-code-attrs-start title = 'Kullanıcı İçin Okunmamış Bildirim Sayısı cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Belirli Sayfa İçin Kullanıcının Okunmamış Bildirim Sayısı cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Bildirim Sayısı İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Kullanıcıya göre filtreleme. **/
    userId?: string
    /** urlId'ye göre filtreleme. **/
    urlId?: string
    /** Kaynak yoruma göre filtreleme. **/
    fromCommentId?: string
    /** Okunmuş/okunmamış durumuna göre filtreleme. **/
    viewed?: 'true' | 'false'
    /** Türüne göre filtreleme. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Bildirim Sayısı Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
    count?: number
}
[inline-code-end]