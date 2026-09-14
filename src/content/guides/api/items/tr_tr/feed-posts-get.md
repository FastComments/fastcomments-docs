[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Bir akıştaki gönderileri, en yenilerden başlayarak alır. Sayfalama imleç tabanlıdır: bir sonraki sayfayı almak için aldığınız son gönderinin `_id` değerini `afterId` olarak gönderin.

Dönen her on gönderi için bir kredi, minimum bir kredi olmak üzere maliyetlidir.

[inline-code-attrs-start title = 'FeedPost cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Bu gönderi kimliğinden sonraki gönderileri döndür. İlk sayfa için atlayın. **/
    afterId?: string
    /** Varsayılan 10'dur. Maksimum 1000. **/
    limit?: number
    /** Yalnızca bu etiketlere sahip gönderileri döndür. Birden fazla etiket için parametreyi tekrarlayın. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]