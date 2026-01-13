[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir SSO kullanıcısını güncelleme yeteneği sağlar.

[inline-code-attrs-start title = 'SSOUser Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Bu örnekte erişim kontrolü için `groupIds` belirtiyoruz, ancak bu isteğe bağlıdır.

[inline-code-attrs-start title = 'SSOUser Güncelleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** E-posta veya kullanıcı adı değiştirildiğinde, kullanıcının yorumlarını da güncellemek için bunu 'true' olarak ayarlayabilirsiniz. Bu kredi maliyetini iki katına çıkaracaktır. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Güncelleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda eklenir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Başarısızlık durumunda eklenir. **/
    reason?: string
    user?: SSOUser; // Başarı durumunda güncellenen kullanıcıyı döndürürüz.
}
[inline-code-end]