[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `Page`'i güncelleme yeteneği sağlar. İlgili yorumlar da güncellenecektir.

[inline-code-attrs-start title = 'Sayfa Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Güncelleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Güncelleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Included on failure. **/
    reason?: string
    user?: Page; // We return the complete updated page on success.
}
[inline-code-end]

#### Not

Sayfa nesnesindeki bazı parametreler otomatik olarak güncellenir. Bunlar sayaçlar (counts) ve `title` öznitelikleridir. Sayaçlar API aracılığıyla güncellenemez
çünkü bunlar hesaplanan değerlerdir. Sayfanın `title` alanı API ile ayarlanabilir, ancak yorum bileşeni aynı `urlId`'ye sahip ve farklı bir sayfa başlığı olan bir sayfada kullanılırsa üzerine yazılır.