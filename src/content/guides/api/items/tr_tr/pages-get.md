[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Şu anda yalnızca hesabınıza bağlı tüm sayfaları (veya `/by-url-id` ile tek bir sayfayı) alabilirsiniz. Daha ayrıntılı arama isterseniz, [bize ulaşın](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Sayfalar cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfalar İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfalar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Yararlı İpucu

`Comment` API'si bir `urlId` gerektirir. Hangi `urlId` değerlerinin size sunulduğunu görmek için önce `Pages` API'sini çağırabilirsiniz.