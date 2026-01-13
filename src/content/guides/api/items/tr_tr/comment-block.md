[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, belirli bir yorumu yazan kullanıcıyı engelleme yeteneği sağlar. FastComments.com Kullanıcıları, SSO Kullanıcıları ve Tenant Kullanıcıları tarafından yazılan yorumlardan engellemeyi destekler.

Ayrıca, bu işlem gerçekleştirildikten sonra istemcide potansiyel olarak görünür başka hangi yorumların engellenmesi/engeli kaldırılması gerektiğini kontrol etmek için bir `commentIdsToCheck` gövde parametresini destekler.

Notlar:

- Bu çağrı her zaman bir kullanıcı bağlamında yapılmalıdır. Kullanıcı FastComments.com Kullanıcısı, SSO Kullanıcısı veya Tenant Kullanıcısı olabilir.
- İstek içindeki `userId`, *engelleme işlemini yapan* kullanıcıdır. Örneğin: `User A`, `User B`'yi Engellemek istiyor. `userId=User A` ve `User B`nin yazdığı yorum kimliğini gönderin.
- Tamamen anonim yorumlar (kullanıcı kimliği yok, e‑posta yok) engellenemez ve bir hata döndürülecektir.

[inline-code-attrs-start title = 'Yorum Engelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Anonim engelleme için bir `anonUserId` belirtmeliyiz. Bu, anonim oturumu temsil eden bir kimlik veya rastgele bir UUID olabilir.
Bu, bir kullanıcı oturum açmamış olsa bile aynı `anonUserId` ile yorumlar alınarak yorumları engellemeyi desteklememizi sağlar.

[inline-code-attrs-start title = 'Anonim Yorum Engelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Yorum Engelleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum Engelleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Eğer commentIdsToCheck tanımlanmışsa, bu eşleştirmede true olan girdiler de engellenir. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---