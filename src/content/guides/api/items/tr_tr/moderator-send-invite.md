[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Bu rota tek bir `Moderator` davet etme imkânı sağlar.

Bir `Moderator`'a davet e-postası göndermek için aşağıdaki kısıtlamalar vardır:
- `Moderator` zaten mevcut olmalıdır.
- `fromName` `100 characters`'den uzun olamaz.

**Notlar:**
- Sağlanan e-postaya sahip bir kullanıcı zaten varsa, kiracınızın yorumlarını yönetmesi için davet edilir.
- Sağlanan e-postaya sahip bir kullanıcı **yoksa**, davet bağlantısı onları hesap oluşturmaya yönlendirecektir.
- Davet `30 days` sonra sona erecektir.

Sadece e-postasını bildiğimiz bir kullanıcı için bir `Moderator` oluşturabiliriz:

[inline-code-attrs-start title = 'Moderator Davet cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Bu, `Bob at TenantName is inviting you to be a moderator...` gibi bir e-posta gönderecektir.

[inline-code-attrs-start title = 'Moderator Davet İsteği Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** Kullanıcıya gönderilen e-posta bu isimden gönderilmiş gibi görünecektir. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Davet Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]