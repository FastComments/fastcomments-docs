[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање једног `TenantUser`.

Креирање `TenantUser` има следећа ограничења:

- `username` је обавезан.
- `email` је обавезан.
- `signUpDate` не може бити у будућности.
- `locale` мора бити у листи [Подржани локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` мора бити јединствен на целој FastComments.com. Ако је ово проблем, препоручујемо коришћење SSO-а.
- `email` мора бити јединствен на целој FastComments.com. Ако је ово проблем, препоручујемо коришћење SSO-а.
- Не смете креирати више tenant корисника него што је дефинисано под `maxTenantUsers` у вашем пакету. 

Можемо креирати `TenantUser` на следећи начин

[inline-code-attrs-start title = 'Пример cURL захтева за TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за креирање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за креирање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Укључено у случају неуспеха. **/
    reason?: string
    tenantUser?: TenantUser; // Враћамо комплетан TenantUser који је креиран у случају успеха.
}
[inline-code-end]

---