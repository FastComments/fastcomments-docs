[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Bu rota tek bir `TenantUser`'a giriş bağlantısı gönderebilme imkanı sağlar.

Kullanıcıları toplu oluştururken ve onlara FastComments.com'a nasıl giriş yapacaklarını anlatmak zorunda olmadığınız durumlarda faydalıdır. Bu, onlara giriş yapmak için süresi dolan bir "sihirli bağlantı" gönderecektir; süresi `30 days` sonra sona erer.

Bir `TenantUser`'a giriş bağlantısı göndermek için aşağıdaki kısıtlamalar geçerlidir:
- `TenantUser` zaten mevcut olmalıdır.
- `TenantUser`'ın ait olduğu `Tenant`'ı yönetme erişiminiz olmalıdır.

Bir `TenantUser`'a giriş bağlantısı şu şekilde gönderilebilir:

[inline-code-attrs-start title = 'TenantUser Giriş Bağlantısı cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Bu, şu şekilde bir e-posta gönderecektir: `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'TenantUser Giriş Bağlantısı İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Giriş Bağlantısı Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]