[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası tek bir yorumu güncelleme yeteneği sağlar.

Notlar:

- Bu API, istenirse yorum widget'ını "canlı" olarak güncelleyebilir (bu, temel `creditsCost` değerini `1`'den `2`'ye yükseltir).
  - Bu, yorumları sayfalar arasında "canlı" olarak taşımayı sağlayabilir (`urlId`'yi değiştirmek).
  - Taşımalar, sayfalar önceden hesaplandığı ve bu CPU yoğun olduğu için ek `2` kredi maliyeti getirir.
- Oluşturma API'sinin aksine, bu API e-posta sağlanmış olsa bile sistemimizde kullanıcı nesnelerini OTOMATİK olarak oluşturmaz.
- Bu API aracılığıyla güncellenen yorumlar istenirse yine de spam için kontrol edilebilir.
- Özelleştirme Kuralı yönetici sayfası aracılığıyla yapılandırılmışsa maksimum yorum uzunluğu gibi yapılandırmalar burada uygulanır.
- Kullanıcıların yorum metinlerini güncellemelerine izin vermek için istek gövdesinde yalnızca `comment`'i belirtebilirsiniz. Ortaya çıkacak `commentHTML`'i biz oluşturacağız.
  - Hem `comment` hem de `commentHTML` belirlerseniz HTML'i otomatik olarak oluşturmayacağız.
  - Kullanıcı yeni metnine bahsetmeler veya hashtag'ler eklerse, bu yine `POST` API'si gibi işlenecektir.
- Bir yorumda `commenterEmail` güncellenirken, `userId`'yi de belirtmek en iyisidir. Aksi takdirde, bu e-postaya sahip kullanıcının kiracınıza ait olduğundan emin olmanız gerekir; aksi halde istek başarısız olur.  


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
	/** Güncellemeyi yapan kullanıcı. İstenirse yorumun düzenleyip düzenleyemeyeceğini kontrol etmek için kullanılabilir.  **/
    contextUserId?: string
	/** Yeni yorumun spam gibi görünüp görünmediğini kontrol etmeli miyiz?  **/
    doSpamCheck?: 'true' | 'false'
	/** Yorumun aynı urlId ile yorum widget'ı örneklerini görüntüleyen kullanıcılara "canlı" olarak görünüp görünmeyeceği. NOT: Kredi maliyetini 1'den 2'ye katlar. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]