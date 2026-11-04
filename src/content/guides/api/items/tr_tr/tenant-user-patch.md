[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Bu rota, tek bir `TenantUser` güncelleme olanağı sağlar.

Bir `TenantUser` güncellemenin aşağıdaki kısıtlamaları vardır:

- `signUpDate` gelecekte bir tarih olamaz.
- `locale`, [Desteklenen Diller](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) listesinde yer almalıdır.
- `username`, tüm FastComments.com genelinde benzersiz olmalıdır. Bu bir sorunsa, bunun yerine SSO kullanmanızı öneririz.
- `email`, tüm FastComments.com genelinde benzersiz olmalıdır. Bu bir sorunsa, bunun yerine SSO kullanmanızı öneririz.
- Bir kullanıcının `tenantId` değerini güncelleyemezsiniz.

Bir `TenantUser`'ı aşağıdaki gibi güncelleyebiliriz

[inline-code-attrs-start title = 'TenantUser Güncelleme cURL Örneği'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Güncelleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Güncelleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]