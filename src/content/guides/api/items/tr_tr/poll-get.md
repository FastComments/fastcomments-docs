[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Bir yoruma eklenmiş anketi, mevcut oy sayılarıyla birlikte okur.

Anketler, yorum API'leri tarafından yorumun kendisinde de döndürülür, bu yüzden yalnızca sonuçları istiyor ve tüm yorumu istemiyorsanız bunu kullanın.

[inline-code-attrs-start title = 'Anket Getir cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Anket Getir İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Anket Getir Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Anketi olmayan bir yorum, silinmiş bir yorum ve var olmayan bir yorum kimliği hepsi aynı şekilde yanıt verir, `poll-not-found` ile.

---