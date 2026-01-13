[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir SSO kullanıcısının oluşturulmasını sağlar.

Aynı ID'ye sahip iki kullanıcı oluşturmaya çalışmak hata ile sonuçlanır.

[inline-code-attrs-start title = 'SSOUser Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Bu örnekte erişim kontrolü için `groupIds` belirtiyoruz, ancak bu isteğe bağlıdır.

[inline-code-attrs-start title = 'SSOUser Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık halinde dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Başarısızlık halinde dahil edilir. **/
    reason?: string
    user?: SSOUser; // Başarı durumunda oluşturulan kullanıcıyı döndürür.
}
[inline-code-end]

#### Entegrasyon Notu

API tarafından gönderilen veriler, farklı bir SSO User HMAC yükü göndererek kolayca geçersiz kılınabilir. Örneğin, kullanıcı adını API üzerinden ayarlarsanız, ancak sayfa yüklemesi sırasında SSO akışıyla farklı bir kullanıcı adı gönderirseniz, kullanıcı adlarını otomatik olarak güncelleyeceğiz.

Bu akışta kullanıcı parametrelerini, bunları açıkça belirtmediğiniz veya null olarak ayarlamadığınız (undefined değil) sürece güncellemeyeceğiz.