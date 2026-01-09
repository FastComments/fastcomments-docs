[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Belirli bir `urlId` üzerinde bir kullanıcı tarafından bırakılan oyları getirmeyi sağlar. Herhangi bir FastComments.com veya `SSO User` olabilecek bir `userId` alır.

Bu, bir kullanıcının bir yoruma oy verip vermediğini göstermek istiyorsanız yararlıdır. Yorumları getirirken, aynı `urlId` ile kullanıcı için aynı anda bu API'yi çağırmanız yeterlidir.

Eğer anonim oy kullanıyorsanız bunun yerine `anonUserId` göndermek isteyeceksiniz.

[inline-code-attrs-start title = 'Kullanıcı için Oylar cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Anonim Kullanıcı için Oylar cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Anonim oyların `appliedAuthorizedVotes` listesinde görüneceğini unutmayın. API anahtarı ile API üzerinden oluşturuldukları için yetkili kabul edilirler.

[inline-code-attrs-start title = 'Kullanıcı için Oylar İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Kullanıcı için Oylar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
    /** Yetkili ve doğrulanmış oylar; ilgili yorumlara uygulanır. **/
    appliedAuthorizedVotes: Vote[]
    /** Doğrulama bekleyen oylar; henüz ilgili yorumlara uygulanmamıştır. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---