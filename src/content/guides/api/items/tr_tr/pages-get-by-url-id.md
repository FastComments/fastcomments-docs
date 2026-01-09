[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages/by-url-id'; creditsCost = 1; api-resource-header-end]

Bireysel sayfalar ilgili `urlId` ile getirilebilir. Bu, sayfa başlıklarını veya yorum sayılarını aramak için faydalı olabilir. 

[inline-code-attrs-start title = 'URL ID ile Sayfa cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/by-url-id?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'URL ID ile Sayfa İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'URL ID ile Sayfa Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    page?: Page[] | null
}
[inline-code-end]

#### Yararlı İpucu

`urlId` gibi değerleri URI Encode (URI Kodlama) yapmayı unutmayın.

---