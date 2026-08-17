[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Овај рут пружа могућност ажурирања једног `TenantUser`.

Ажурирање `TenantUser`-а има следећа ограничења:

- `signUpDate` не сме бити у будућности.
- `locale` мора бити у листи [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` мора бити јединствен на целом FastComments.com. Ако је ово проблем, предлажемо да уместо тога користите SSO.
- `email` мора бити јединствен на целом FastComments.com. Ако је ово проблем, предлажемо да уместо тога користите SSO.
- Не можете ажурирати `tenantId` корисника.

Можемо креирати `TenantUser` на следећи начин

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за ажурирање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Када се мењају email или username, можете поставити ово на true да такође ажурирате коментаре корисника. Ово ће удвостручити цену кредита. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за ажурирање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---