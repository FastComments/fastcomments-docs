[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Bu API, `skip`, `before` ve `after` parametreleriyle sağlanan sayfalandırmayı kullanır. AuditLogs, `1000` öğelik sayfalar halinde, `when` ve `id` sırasına göre döndürülür.

Her `1000` logu almak kredi maliyeti `10`'dur.

Varsayılan olarak, size **en yeni öğeler ilk** olacak şekilde bir liste verilir. Bu şekilde, `skip=0` ile başlayıp, tükettiğiniz son kaydı bulana kadar sayfalandırma yapabilirsiniz.

Alternatif olarak, en eski ilk olacak şekilde sıralayabilir ve kayıt kalmayana kadar sayfalandırabilirsiniz.

Sıralama, `order`'ı `ASC` veya `DESC` olarak ayarlayarak yapılabilir. Varsayılan `ASC`'dir.

Tarihe göre sorgulama, `before` ve `after`'ı milisaniye içeren zaman damgaları olarak kullanarak mümkündür. `before` ve `after` dahil değildir.

[inline-code-attrs-start title = 'AuditLog cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık halinde eklenir. **/
    reason?: string
    /** Loglar! **/
    auditLogs: AuditLog[]
}
[inline-code-end]