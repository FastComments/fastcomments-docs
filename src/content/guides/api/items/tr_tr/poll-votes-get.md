[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Bir anketin sayımlarının arkasındaki bireysel oyları listeler, en eski oylardan başlayarak. Döndürülen her 100 oy için bir kredi.

Bir anket bir yoruma aittir, bu yüzden oylar bir seferde bir anket olarak okunur ve `commentId` gereklidir. `voterId` ile bir kişinin nasıl oy kullandığını kontrol ederek, ya da `optionId` ile belirli bir seçeneği seçen herkesi listeleyerek daha da daraltabilirsiniz.

Her çağrıda en fazla 1000 oy döndürülür. Daha fazlasını sayfalamak için `skip` kullanın.

Anketin `privacy` ayarı dikkate alınır: anonim bir anketteki oylar okunamaz ve istek `poll-anonymous` hatasıyla başarısız olur. Ayrıntılar için `PollVote` yapısına bakın.

[inline-code-attrs-start title = 'PollVotes Get cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Seçenek Başına Oyları Sayma

Sonuçları elde etmek için bunları toplamanıza gerek yok - anket kendi sayımlarını taşır. Bunun yerine anketi `GET /api/v1/polls/:commentId` ile okuyun ve kimin oy kullandığını bilmeniz gerektiğinde bu API'yi kullanın.

### Bir Sayfadaki Tüm Anketler

Sayfa genelinde bir oy listesi yoktur. Tüm bir sayfayı raporlamak için yorumlarını `GET /api/v1/comments` ile alın; bu, her yorumun anketini ve sayımlarını döndürür ve ardından ilgilendiğiniz anketlerin oylarını okuyabilirsiniz.