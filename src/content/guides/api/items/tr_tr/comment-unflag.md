[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, belirli bir kullanıcı için bir yorumun bayrağını kaldırma yeteneği sağlar.

Notlar:

- Bu çağrı her zaman bir kullanıcı bağlamında yapılmalıdır. Kullanıcı, FastComments.com Kullanıcısı, SSO Kullanıcısı veya Kiracı (Tenant) Kullanıcısı olabilir.
- Bir yorum otomatik olarak onaydan çıkarıldıktan (gizlendikten) sonra — yorum yalnızca bir yönetici veya moderatör tarafından yeniden onaylanabilir. Bayrağın kaldırılması yorumu yeniden onaylamaz.

[inline-code-attrs-start title = 'Yorum Bayrağını Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Anonim bayraklama için bir `anonUserId` belirtmeliyiz. Bu, anonim oturumu temsil eden bir kimlik olabilir veya rastgele bir UUID olabilir.

[inline-code-attrs-start title = 'Anonim Yorum Bayraklama cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Yorum Bayrağını Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum Bayrağını Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]