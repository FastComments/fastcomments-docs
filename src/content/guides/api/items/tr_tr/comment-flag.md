[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, belirli bir kullanıcı için bir yorumu işaretleme (flag) yeteneği sağlar.

Notes:

- Bu çağrı her zaman bir kullanıcı bağlamında yapılmalıdır. Kullanıcı FastComments.com Kullanıcısı, SSO Kullanıcısı veya Tenant Kullanıcısı olabilir.
- Eğer bir flag-to-hide eşiği ayarlanmışsa, yorum tanımlanan sayıda işaretlendikten sonra canlı olarak otomatik gizlenecektir.
- Otomatik olarak onayı kaldırıldıktan (gizlendikten) sonra - yorum yalnızca bir yönetici veya moderatör tarafından yeniden onaylanabilir. İşaretin kaldırılması yorumu yeniden onaylamaz.

[inline-code-attrs-start title = 'Yorum İşaretleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Anonim işaretleme için bir `anonUserId` belirtmemiz gerekir. Bu, anonim oturumu temsil eden bir kimlik veya rastgele bir UUID olabilir. Bu, bir kullanıcı giriş yapmamış olsa bile yorumları işaretleme ve işaret kaldırmayı desteklememizi sağlar. Bu şekilde, aynı `anonUserId` ile yorumlar alındığında yorum işaretlenmiş olarak işaretlenebilir.

[inline-code-attrs-start title = 'Anonim Yorum İşaretleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Yorum İşaretleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum İşaretleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    /** Yorum belirlenen sayıda işaretlendiği için onayı kaldırıldı (gizlendi) mı? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---