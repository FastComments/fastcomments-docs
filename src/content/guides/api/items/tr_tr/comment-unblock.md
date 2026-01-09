[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, belirli bir yorumu yazmış bir kullanıcının engellemesini kaldırma yeteneği sağlar. FastComments.com Users, SSO Users, and Tenant Users tarafından yazılmış yorumların engellemesini kaldırmayı destekler.

Bu işlem gerçekleştirildikten sonra istemcideki diğer potansiyel olarak görünür yorumların engellenip/engellenmeyeceğini kontrol etmek için `commentIdsToCheck` gövde parametresini destekler.

Notlar:

- Bu çağrı her zaman bir kullanıcı bağlamında yapılmalıdır. Kullanıcı FastComments.com User, SSO User, veya Tenant User olabilir.
- İstek içindeki `userId`, engelleme kaldırma işlemini yapan kullanıcıdır. Örneğin: `User A` `User B`'nin engellemesini kaldırmak istiyor. `userId=User A` ve `User B` tarafından yazılan yorumun id'sini gönderin.
- Tamamen anonim yorumlar (kullanıcı kimliği yok, e-posta yok) engellenemez ve bir hata döndürülecektir.

[inline-code-attrs-start title = 'Yorumun Engellenmesini Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonim Yorumun Engellenmesini Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Yorumun Engellenmesini Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorumun Engellenmesini Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Eğer commentIdsToCheck tanımlıysa, bu haritadaki true olan girdiler hâlâ engellenmiştir. False ise, kullanıcıların sayfayı yenilemek zorunda kalmamaları için yorumların görünürlüğünü geri açmak isteyebilirsiniz. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]