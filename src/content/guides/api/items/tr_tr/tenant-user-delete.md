[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Bu rota bir `TenantUser`'ı id ile kaldırmayı sağlar.

Kullanıcının yorumlarının silinmesi `deleteComments` sorgu parametresi ile mümkündür. Bunun true olması durumunda:

1. Kullanıcının tüm yorumları canlı olarak silinecektir.
2. Tüm __child__ (şimdi öksüz) yorumlar, her yorumun ilişkili sayfa yapılandırmasına göre silinecek veya anonimleştirilecektir. Örneğin konu silme modu "anonymize" ise yanıtlar kalır ve kullanıcının yorumları anonimleştirilir. Bu yalnızca `commentDeleteMode` `Remove` olduğunda (varsayılan değer) geçerlidir.
3. `creditsCost` değeri `2` olur.

### Anonymized Comments

Kullanıcının yorumlarını saklayabilir fakat `commentDeleteMode=1` ayarlayarak bunları yalnızca anonimleştirebilirsiniz.

Kullanıcının yorumları anonimleştirildiğinde aşağıdaki değerler null olarak ayarlanır:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` ve `isDeletedUser` `true` olarak ayarlanır.

Render edilirken, yorum widget'ı kullanıcının adı için `DELETED_USER_PLACEHOLDER` (varsayılan: "[deleted]") ve yorum için `DELETED_CONTENT_PLACEHOLDER` kullanır. Bunlar Widget Özelleştirme UI'sı aracılığıyla özelleştirilebilir.

### Examples

[inline-code-attrs-start title = 'TenantUser Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // varsayılan
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Kullanıcının yorumlarını aynı zamanda silmek için bunu true olarak ayarlayabilirsiniz. Bu kredi maliyetini iki katına çıkarır. **/
    deleteComments?: 'true' | 'false'
    /** Kullanıcının yorumlarının nasıl ele alınacağını belirlemek için bunu istediğiniz gibi ayarlayabilirsiniz. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]