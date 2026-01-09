[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası yorum oluşturma yeteneği sağlar.

Yaygın kullanım durumları özel kullanıcı arayüzleri, entegrasyonlar veya içe aktarımlardır.

Notlar:

- Bu API, istenirse yorum widget'ını "canlı" olarak güncelleyebilir (bu, `creditsCost`'u `1`'den `2`'ye çıkarır).
- E-posta sağlanırsa bu API sistemimizde otomatik olarak kullanıcı nesneleri oluşturur.
- Farklı e-postalara sahip iki yorumu, fakat aynı kullanıcı adıyla kaydetmeye çalışmak, ikinci yorum için bir hata ile sonuçlanır.
- Eğer `parentId` belirtiyorsanız ve bir alt yorumun `notificationSentForParent` değeri false ise, **ebeveyn yorum için bildirimler göndereceğiz**. Bu her saat yapılır (gönderilen e-posta sayısını azaltmak için bildirimleri toplu olarak gönderiyoruz).
- Kullanıcı oluşturulurken karşılama e-postaları veya yorum doğrulama e-postaları göndermek istiyorsanız, sorgu parametrelerinde `sendEmails` değerini `true` olarak ayarlayın.
- Bu API aracılığıyla oluşturulan yorumlar yönetici uygulamasının Analytics ve Moderation sayfalarında görünecektir.
- Ayar açık ise, "bad words" yorumcu isimlerinde ve yorum metninde hâlâ maskelenir.
- Bu API ile oluşturulan yorumlar istenirse yine de spam için kontrol edilebilir.
- Özelleştirme Kuralı yönetici sayfası aracılığıyla yapılandırıldıysa, maksimum yorum uzunluğu gibi yapılandırmalar burada uygulanır.

Yorum widget'ında görüntülenecek şekilde göndermek için gereken minimum veriler aşağıdaki gibidir:

[inline-code-attrs-start title = 'Minimum Yorum POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Daha gerçekçi bir istek şöyle görünebilir:

[inline-code-attrs-start title = 'Yorum POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Yorum POST İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Yorumun aynı urlId'ye sahip yorum widget örneklerini görüntüleyen kullanıcılara "canlı" olarak görünmesi gerekip gerekmediği. NOT: Kredi maliyetini 1'den 2'ye çıkarır. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Yorum POST Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
    /** Oluşturulan yorum. **/
    comment?: Comment
    /** İlişkili kullanıcı; önceden var olup olmadığı değişebilir. **/
    user?: User
}
[inline-code-end]

---