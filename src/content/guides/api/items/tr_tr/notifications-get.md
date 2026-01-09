[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Bu rota en yeni önce olacak şekilde `createdAt`'e göre sıralanmış en fazla 30 `Notification` nesnesi döndürür.

`userId` ile filtreleyebilirsiniz. SSO ile, kullanıcı kimliği `<tenant id>:<user id>` formatındadır.

[inline-code-attrs-start title = 'Kullanıcı İçin Okunmamış Bildirimler cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Bildirimler İçin İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Kayıtları atlayarak sayfalandırma. **/
    skip?: number
    /** Kullanıcıya göre filtrele. **/
    userId?: string
    /** urlId'ye göre filtrele. **/
    urlId?: string
    /** Kaynak yoruma göre filtrele. **/
    fromCommentId?: string
    /** Okundu/okunmadı durumuna göre filtrele. **/
    viewed?: 'true' | 'false'
    /** Türe göre filtrele. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Bildirimler İçin Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---