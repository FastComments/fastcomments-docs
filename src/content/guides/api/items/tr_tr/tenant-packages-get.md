[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Bu API, `skip` sorgu parametresiyle sağlanan sayfalandırmayı kullanır. TenantPackages, `createdAt` ve `id`'ye göre sıralanmış olarak, `100` öğelik sayfalar halinde döndürülür.

Maliyet, döndürülen tenant paketlerinin sayısına göre hesaplanır; döndürülen tenant paketleri için maliyet `1 credit per 10`'dur.

[inline-code-attrs-start title = 'TenantPackage cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalandırma için atlanacak tenant paketlerinin sayısı. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---