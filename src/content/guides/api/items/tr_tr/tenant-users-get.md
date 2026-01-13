[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Bu API, sayfalama için `skip` sorgu parametresi ile çalışır. TenantUsers, `100` öğe sayfaları halinde döndürülür; `signUpDate`, `username` ve `id`'ye göre sıralanır.

Maliyet döndürülen tenant users sayısına göre belirlenir; döndürülen her 10 tenant users için `1 credit per 10` ücret alınır.

[inline-code-attrs-start title = 'TenantUser cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalama için atlanacak tenant kullanıcı sayısı. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Hata durumunda dahil edilir. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---