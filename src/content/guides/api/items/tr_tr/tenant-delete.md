[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Bu rota, bir `Tenant` **ve ilgili tüm verilerin** (kullanıcılar, yorumlar, vb.) id ile kaldırılmasını sağlar.

Aşağıdaki kısıtlamalar tenant kaldırma işlemi ile ilgilidir:

- Tenant sizin kendi tenant'ınız olmalı veya sizin yönettiğiniz beyaz etiketli bir tenant olmalıdır.
- `sure` sorgu parametresi `true` olarak ayarlanmış olmalıdır.

[inline-code-attrs-start title = 'Kiracı Silme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Kiracı Silme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Kiracı Silme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
}
[inline-code-end]