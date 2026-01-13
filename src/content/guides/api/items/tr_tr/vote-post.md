[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir yetkili `Vote` ekleme yeteneği sağlar. Oylar `up` (+1) veya `down` (-1) olabilir.

[inline-code-attrs-start title = 'Oy Oluşturma cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonim Oy Oluşturma cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Oy Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Oy Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Anonim Oy Oluşturma

Anonim oylar, sorgu parametrelerinde `userId` yerine `anonUserId` ayarlanarak oluşturulabilir.

Bu id herhangi bir kullanıcı nesnesiyle eşleşmek zorunda değildir (dolayısıyla anonim). Bu, yalnızca oturum için bir tanımlayıcıdır, böylece aynı oturum içinde oyları tekrar alabilir ve bir yorumun oylandığını kontrol edebilirsiniz.

Eğer FastComments'ın sahip olduğu gibi "anonim oturumlar" gibi bir şeye sahip değilseniz - bunu basitçe rastgele bir ID'ye, örneğin bir UUID'ye ayarlayabilirsiniz (ancak alan tasarrufu için daha küçük tanımlayıcıları tercih ediyoruz).

### Diğer Notlar

- Bu API, tenant düzeyindeki ayarlara uyar. Örneğin, belirli bir sayfa için oylamayı devre dışı bırakırsanız ve API aracılığıyla bir oy oluşturmaya çalışırsanız, `voting-disabled` hata kodu ile başarısız olur.
- Bu API varsayılan olarak canlıdır.
- Bu API, ilgili `Comment`'in `votes` alanını güncelleyecektir.