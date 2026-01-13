[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `TenantUser`'ı değiştirme yeteneği sağlar.

Bir `TenantUser`'ı değiştirme işleminin aşağıdaki kısıtlamaları vardır:

- `signUpDate` gelecekte olamaz.
- `locale`, [Desteklenen Yereller](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) listesinden biri olmalıdır.
- `username`, FastComments.com genelinde benzersiz olmalıdır. Bu bir sorun ise, yerine SSO kullanmanızı öneririz.
- `email`, FastComments.com genelinde benzersiz olmalıdır. Bu bir sorun ise, yerine SSO kullanmanızı öneririz.
- Bir kullanıcının `tenantId`'sini güncelleyemezsiniz.

Bir `TenantUser` aşağıdaki şekilde oluşturabiliriz

[inline-code-attrs-start title = 'TenantUser Değiştirme cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Değiştirme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** E-posta veya kullanıcı adı değiştirildiğinde, kullanıcının yorumlarını da güncellemek için bunu true olarak ayarlayabilirsiniz. Bu kredi maliyetini iki katına çıkarır. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Değiştirme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]