[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Bu API, `skip`, `limit`, `before` ve `after` parametreleriyle sağlanan sayfalama kullanır. AuditLog'lar varsayılan olarak `5000` kayıtlık sayfalarda döndürülür, maksimum `limit` değeri `10000` olabilir ve `when` ve `id` alanına göre sıralanır. Sayfalar büyük olur çünkü bu uç nokta genellikle geçmişi dökmek için kullanılır, etkileşimli olarak sayfalama yapmak için değil.

Dönen her `100` kayıt için kredi maliyeti `1`'dir.

Varsayılan olarak, **en yeni öğeler önce** listelenir. Bu sayede `skip=0` ile başlayıp, tükettiğiniz son kaydı bulana kadar sayfalayarak sorgu yapabilirsiniz.

Alternatif olarak, en eski öğeler önce sıralanabilir ve kayıt kalmayana kadar sayfalayabilirsiniz.

Sıralama, `order` parametresini `ASC` veya `DESC` olarak ayarlayarak yapılabilir. Varsayılan değer `DESC`'tir.

Tarih sorgulaması, milisaniye cinsinden zaman damgalarıyla `before` ve `after` parametreleriyle mümkündür. `before` ve `after` KAPSAMLI DEĞİLDİR ve yalnızca biri tek başına kullanılabilir.

## Bir kişinin ne olduğunu bulma

Her olay, kim tarafından gerçekleştirildiğini (`username`, `userId`, `ip`) ve ayrıca ne üzerinde gerçekleştirildiğini kaydeder. `targetLabel`, o nesne için insan tarafından okunabilir bir etikettir; örneğin `jsmith (jsmith@example.com)`. `targetId` ise onun kimliğidir. Bir kişinin adını veya e‑postasını biliyor ancak kimliğini bilmiyorsanız, etikete büyük/küçük harfe duyarsız alt dize eşleşmesi için `target` kullanın.

Silme işlemleri, olay anındaki etiketi yakalar, böylece temel kayıt silinmiş olsa bile kaldırılan kullanıcı veya moderatör hâlâ tanımlanabilir.

## Yönetilen kiracılar

Kiracınız diğer kiracıları yönetiyorsa, `includeManagedTenants=true` ayarlayarak bir yanıt içinde kendi kiracınız ve yönettiği tüm kiracılardan gelen olayları döndürebilirsiniz. Döndürülen her kaydın `tenantId` alanı, kaydın hangi kiracıdan geldiğini gösterir.

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
    /** Maksimum 10000. Varsayılan 5000. **/
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