[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Bu rota, tek bir SSO kullanıcısını id'si ile kaldırmayı sağlar.

Bu kullanıcının payload'ını içeren yorum bileşenini yeniden yüklemenin kullanıcıyı sorunsuz şekilde yeniden oluşturacağını unutmayın.

Kullanıcının yorumlarını silmek `deleteComments` sorgu parametresi aracılığıyla mümkündür. Bunun true olması durumunda:

1. Kullanıcının tüm yorumları canlı olarak silinecektir.
2. Tüm __child__ (şimdi yetim) yorumlar, her bir yorumun ilişkili olduğu sayfa yapılandırmasına göre silinecek veya anonimleştirilecektir. Örneğin thread deletion modu "anonymize" ise yanıtlar kalacak ve kullanıcının yorumları anonimleştirilecektir. Bu yalnızca `commentDeleteMode` `Remove` (`varsayılan değer`) olduğunda geçerlidir.
3. `creditsCost` değeri `2` olur.

### Anonimleştirilmiş Yorumlar

Kullanıcının yorumlarını koruyabilir ancak `commentDeleteMode=1` olarak ayarlayarak bunları anonimleştirebilirsiniz.

Kullanıcının yorumları anonimleştirildiyse aşağıdaki değerler null olarak ayarlanır:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` ve `isDeletedUser` `true` olarak ayarlanır.

Render sırasında yorum bileşeni, kullanıcının adı için `DELETED_USER_PLACEHOLDER` (varsayılan: "[deleted]") ve yorum için `DELETED_CONTENT_PLACEHOLDER` kullanır. Bunlar Widget Özelleştirme UI'si aracılığıyla özelleştirilebilir.

### Örnekler

[inline-code-attrs-start title = 'SSOUser Kaldırma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Kaldırma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // varsayılan
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Kullanıcının yorumlarını da silmek için bunu true olarak ayarlayabilirsiniz. Bu kredi maliyetini iki katına çıkarır. **/
    deleteComments?: 'true' | 'false'
    /** Kullanıcının yorumlarını nasıl işleyeceğinizi belirlemek için bunu istediğiniz gibi ayarlayabilirsiniz. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Kaldırma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    user?: SSOUser; // Başarılı olduğunda kaldırılan kullanıcıyı döneriz.
}
[inline-code-end]