[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `TenantUser` ekleme olanağı sağlar.

Bir `TenantUser` oluşturmanın aşağıdaki kısıtlamaları vardır:

- `username` gereklidir.
- `email` gereklidir.
- `signUpDate` gelecekte olamaz.
- `locale` şu listede olmalıdır: [Desteklenen Yereller](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` FastComments.com genelinde benzersiz olmalıdır. Bu bir sorun ise, bunun yerine SSO kullanmanızı öneririz.
- `email` FastComments.com genelinde benzersiz olmalıdır. Bu bir sorun ise, bunun yerine SSO kullanmanızı öneririz.
- Paketinizde `maxTenantUsers` altında tanımlanan sayıda tenant kullanıcıdan daha fazlasını oluşturamazsınız. 

`TenantUser` şu şekilde oluşturabiliriz

[inline-code-attrs-start title = 'TenantUser Oluşturma cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    tenantUser?: TenantUser; // Başarı durumunda oluşturulan tenant kullanıcıyı eksiksiz döneriz.
}
[inline-code-end]

---