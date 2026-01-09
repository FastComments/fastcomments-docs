[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава замјену појединачног `TenantUser`.

Замјена `TenantUser` има следећа ограничења:

- Датум `signUpDate` не смије бити у будућности.
- `locale` мора бити на листи [Подржани локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` мора бити јединствен на целом FastComments.com. Ако је то проблем, препоручујемо коришћење SSO-а.
- `email` мора бити јединствен на целом FastComments.com. Ако је то проблем, препоручујемо коришћење SSO-а.
- Не можете ажурирати `tenantId` корисника.

Можемо креирати `TenantUser` на следећи начин

[inline-code-attrs-start title = 'Пример cURL захтјева за замјену TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за замјену TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Када се промијени email или username, можете поставити ово на true да бисте такође ажурирали корисникове коментаре. Ово ће удвостручити трошак кредита. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за замјену TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]

---