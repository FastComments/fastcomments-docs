[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Bu API, tenant'ınız tarafından yönetilen tenants listesini döndürür.

Sayfalama `skip` sorgu parametresi ile sağlanır. Tenants, `100` öğelik sayfalar halinde, `signUpDate` ve `id`'ye göre sıralanarak döndürülür.

Maliyet, döndürülen tenants sayısına göre belirlenir; döndürülen her 10 tenants için `1 credit per 10` ücretlendirilir.

[inline-code-attrs-start title = 'Tenant cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

`Tenant` nesneleri üzerinde `meta` parametreleri tanımlayabilir ve eşleşen tenants için sorgu yapabilirsiniz. Örneğin, `someKey` anahtarı ve `some-value` meta değeri için, bu anahtar/değer çiftini içeren bir JSON nesnesi oluşturup filtrelemek için bunu URI kodlayarak sorgu parametresi olarak kullanabiliriz:

[inline-code-attrs-start title = 'Meta ile Tenant Sorgulama cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalama için atlanacak tenants sayısı. **/
    skip?: number
    /** Meta parametrelerine göre filtreleme. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Hata durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Hata durumunda eklenir. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]