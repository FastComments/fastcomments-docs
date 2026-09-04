[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Bu API, `skip`, `limit`, `before` ve `after` parametreleriyle sağlanan sayfalama kullanır. AuditLog'lar varsayılan olarak `100`'lük sayfalarda döndürülür, maksimum `limit` `200`'e kadar, `when` ve `id`'ye göre sıralanır.

Dönen her `100` log için kredi maliyeti `1`'dir.

Varsayılan olarak, **en yeni öğeler önce** bir liste alırsınız. Bu şekilde, `skip=0` ile başlayarak sorgulayabilir, tükettiğiniz son kaydı bulana kadar sayfalayabilirsiniz.

Alternatif olarak, en eski öğeler önce sıralayabilir ve daha fazla kayıt kalmayana kadar sayfalayabilirsiniz.

`order` parametresini `ASC` veya `DESC` olarak ayarlayarak sıralama yapabilirsiniz. Varsayılan `DESC`'tir.

Tarih sorgulaması, milisaniye cinsinden zaman damgaları olarak `before` ve `after` ile mümkündür. `before` ve `after` KAPSAMLI DEĞİLDİR ve her biri tek başına kullanılabilir.

## Bir kişinin başına ne geldiğini bulma

Her olay, kim tarafından gerçekleştirildiğini (`username`, `userId`, `ip`) ve ayrıca ne üzerinde gerçekleştirildiğini kaydeder. `targetLabel` bu nesne için insan tarafından okunabilir bir etikettir, örneğin `jsmith (jsmith@example.com)`, ve `targetId` onun kimliğidir. Bir kişinin adını veya e-posta adresini biliyor ancak kimliğini bilmiyorsanız, etiketteki büyük/küçük harfe duyarsız alt dize eşleşmesi için `target` kullanın.

Silme işlemleri, olay anındaki etiketi yakalar, böylece kaldırılan bir kullanıcı veya moderatör, temel kayıt silinmiş olsa bile hâlâ tanımlanabilir.

## Yönetilen kiracılar

Kiracınız diğer kiracıları yönetiyorsa, `includeManagedTenants=true` ayarlayarak bir yanıt içinde kendi kiracınız ve yönettiği tüm kiracılardan gelen olayları döndürebilirsiniz. Döndürülen her log'un `tenantId` değeri, olayın hangi kiracıdan geldiğini gösterir.

[inline-code-attrs-start title = 'AuditLog cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Yalnızca bu kullanıcı adı tarafından gerçekleştirilen olaylar. **/
    username?: string
    /** Yalnızca bu IP adresinden gelen olaylar. **/
    ip?: string
    /** Yalnızca bu türdeki olaylar. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Yalnızca bu kaynak için olaylar, ör. Kullanıcı veya Moderatör. **/
    resourceName?: string
    /** Yalnızca etkilenen nesnenin bu kimliğe sahip olduğu olaylar. **/
    targetId?: string
    /** Etkilenen nesnenin etiketinde büyük/küçük harfe duyarsız alt dize eşleşmesi. **/
    target?: string
    /** Ayrıca bu kiracının yönettiği kiracılardan gelen olayları da döndür. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    /** Günlükler! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---