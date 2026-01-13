[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Oylar `urlId` ile alınmalıdır.

### Oy Türleri

Üç tür oy vardır:

- Kimliği doğrulanmış oylar, ilgili yoruma uygulanır. Bunları bu API aracılığıyla oluşturabilirsiniz.
- Kimliği doğrulanmış oylar, doğrulama için **beklemede** olan ve bu nedenle henüz yoruma uygulanmamış olanlar. Bunlar bir kullanıcı FastComments.com *oy vermek için giriş yap* mekanizmasını kullandığında oluşturulur.
- Anonim oylar, ilgili yoruma uygulanır. Bunlar anonim yorumla birlikte oluşturulur.

Bunlar karışıklığı azaltmak için API'de ayrı listeler halinde döndürülür.

[inline-code-attrs-start title = 'Oylar cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Oylar İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Oylar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Yetkili, doğrulanmış oylar, ilgili yorumlarına uygulanmış. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonim oylar, ilgili yorumlarına uygulanmış. **/
    appliedAnonymousVotes: Vote[]
    /** Doğrulama bekleyen oylar, henüz ilgili yorumlarına uygulanmamış. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Anonim Oylar Notları

Bu API aracılığıyla oluşturulan anonim oyların `appliedAuthorizedVotes` listesinde görüneceğini unutmayın. API anahtarıyla API aracılığıyla oluşturuldukları için yetkili olarak kabul edilirler.

`appliedAnonymousVotes` yapısı e-posta, API anahtarı vb. olmadan oluşturulan oylar içindir.