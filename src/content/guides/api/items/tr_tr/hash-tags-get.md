[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Bu API, `page` sorgu parametresi ile sağlanan sayfalama kullanır. HashTag'ler `tag`e göre sıralanmış şekilde, `100` öğelik sayfalar halinde döndürülür.

[inline-code-attrs-start title = 'HashTag cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Alınacak sayfa, 0'dan başlar. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    /** Hashtag'ler! **/
    hashTags: HashTag[]
}
[inline-code-end]