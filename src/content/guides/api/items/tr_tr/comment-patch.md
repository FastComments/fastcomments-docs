[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası tek bir yorumu güncelleme olanağı sağlar.

Notlar:

- İstenirse bu API, yorum widget'ını "live" olarak güncelleyebilir (bu, temel `creditsCost`u `1`'den `2`'ye yükseltir).
  - Bu, sayfalar arasında yorumları "live" olarak taşımayı sağlayabilir (`urlId` değiştirerek).
  - Sayfalar önceden hesaplandığı ve bunun CPU açısından maliyetli olduğu için taşımalar ek `2` kredi ücreti gerektirir.
- Oluşturma API'sinin aksine, bu API e-posta sağlansa bile sistemimizde otomatik olarak kullanıcı nesneleri oluşturmaz.
- Bu API ile güncellenen yorumlar istenirse hala spam için kontrol edilebilir.
- Maksimum yorum uzunluğu gibi yapılandırmalar, eğer Customization Rule yönetim sayfası üzerinden yapılandırıldıysa burada uygulanır.
- Kullanıcıların yorum metinlerini güncellemelerine izin vermek için istek gövdesinde yalnızca `comment`'i belirtebilirsiniz. Oluşan `commentHTML`'i biz oluşturacağız.
  - Hem `comment` hem de `commentHTML` tanımlarsanız HTML otomatik olarak oluşturulmaz.
  - Kullanıcı yeni metnine mentions veya hashtags eklerse, bu yine `POST` API'si gibi işlenecektir.
- Bir yorumda `commenterEmail`'i güncellerken, ayrıca `userId`'yi belirtmek en iyisidir. Aksi takdirde, bu e-postaya sahip kullanıcının kiracınıza ait olduğundan emin olmalısınız; aksi halde istek başarısız olur.  
- Hedef yorum kilitliyse (`isLocked: true`), istek `code: 'locked'` ile reddedilir. Önce yorumu kilidini açın, güncelleyin, ardından isterseniz tekrar kilitleyin.


[inline-code-attrs-start title = 'Minimum Yorum PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Yorum PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Güncelleme yapan kullanıcı. İstenirse yorumı düzenleme yetkisini kontrol etmek için kullanılabilir.  **/
    contextUserId?: string
	/** Yeni yorumun spam gibi görünüp görünmediğini kontrol etmeli miyiz?  **/
    doSpamCheck?: 'true' | 'false'
	/** Aynı urlId'ye sahip yorum widget örneklerini görüntüleyen kullanıcılara yorumun "live" olarak görünüp görünmeyeceği. NOT: Kredi maliyetini 1'den 2'ye iki katına çıkarır. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]