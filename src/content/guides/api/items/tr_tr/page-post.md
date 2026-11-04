[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası, sayfa oluşturma yeteneği sağlar.

Yaygın kullanım senaryolarından biri erişim kontrolüdür.

Notlar:

- Bir yorum dizisine yorum yaptıysanız veya bir `Comment` oluşturmak için API'yi çağırdıysanız, zaten bir `Page` nesnesi oluşturmuşsunuzdur! Bunu, yorum widget'ına iletilen aynı `urlId` değerini geçirerek `/by-url-id` `Page` rotası üzerinden getirmeyi deneyebilirsiniz.
- `Page` yapısı bazı **hesaplanmış** değerler içerir.
  Şu anda bunlar `commentCount` ve `rootCommentCount` değerleridir.
  Bunlar otomatik olarak doldurulur ve API aracılığıyla ayarlanamaz. Bunu denemek, API'nin bir hata döndürmesine neden olur.

[inline-code-attrs-start title = 'Page POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Oluşturulan sayfa. **/
    page?: Page
}
[inline-code-end]