[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Bu API endpoint'i, bir `TenantPackage` kaydını `id` üzerinden güncelleme olanağı sağlar.

Bir `TenantPackage` güncellemenin aşağıdaki kısıtlamaları vardır:

- `hasFlexPricing` değerini true olarak ayarlıyorsanız, aynı istekte tüm `flex*` parametreleri zorunludur.
- `name` değeri `50 karakterden` uzun olamaz.
- Her bir `forWhoText` öğesi `200 karakterden` uzun olamaz.
- Her bir `featureTaglines` öğesi `100 karakterden` uzun olamaz.
- `TenantPackage`, üst tenant'tan "daha küçük" olmalıdır. Örneğin, tüm `max*` parametreleri üst tenant'taki değerlerden daha düşük değerlere sahip olmalıdır.
- Bir `TenantPackage` ile ilişkili `tenantId` değerini değiştiremezsiniz.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]