[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Bir ankette oy kaydeder.

Bir seçmenin bir ankette en fazla bir oyu vardır. Aynı seçmen için bu isteği tekrar çağırmak, ikinci bir oy eklemek yerine oyunu yeni seçeneğe taşır ve zaten seçtikleri seçeneğe oy vermek hiçbir şey yapmaz.

Yanıt ankete dahildir, böylece ikinci bir istek yapmadan güncellenmiş sayımları alırsınız.

[inline-code-attrs-start title = 'PollVote Oluştur cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anonim PollVote Oluştur cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** userId veya anonUserId'den birinin sağlanması gerekir. **/
    userId?: string
    anonUserId?: string
    /** Son kullanıcının IP'si, anonim oran sınırlaması için kullanılır. Varsayılan olarak çağıranın IP'si alınır. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    pollVote: PollVote
    /** Güncellenmiş sayımları içeren anket. **/
    poll: CommentPoll
}
[inline-code-end]

### Anonim Oylar

`userId` yerine `anonUserId` ayarlayarak oturum açmamış bir kişi için oy kaydedin. Bu kimlik herhangi bir kullanıcıyla eşleşmek zorunda değildir - sadece oturumu tanımlar, böylece aynı kişi iki kez sayılmaz.

Anonim oylama sitenizde etkinleştirilmiş olmalıdır. Oylama yalnızca oturum açmış kullanıcılarla sınırlıysa, sadece `anonUserId` içeren bir oy `poll-login-required` hatasıyla başarısız olur.

Anonim oylar ayrıca anket başına IP başına oran sınırlamasına tabidir, böylece bir kişinin oturumunu temizleyerek bir anketi doldurması engellenir. Sınırlamanın sizin sunucunuza değil, son kullanıcının `ip`'sine uygulanması için son kullanıcının `ip`'sini gönderin.

### Diğer Notlar

- `userId` sitenizde mevcut bir kullanıcı olmalıdır. Başka bir siteye ait bir kullanıcı için oylar reddedilir.
- Kapalı bir ankette oylama `poll-closed` hatasıyla başarısız olur.
- Bu API, anketteki sayımları günceller ve bağlı widget'lara canlı olarak gönderir.

---