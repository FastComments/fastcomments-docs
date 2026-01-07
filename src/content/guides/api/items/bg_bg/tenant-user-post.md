[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя възможност за добавяне на единичен `TenantUser`.

Създаването на `TenantUser` има следните ограничения:

- `username` е задължително.
- `email` е задължително.
- `signUpDate` не може да бъде в бъдещето.
- `locale` трябва да бъде в списъка на [Поддържани локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` трябва да бъде уникално в целия FastComments.com. Ако това е проблем, предлагаме да използвате SSO вместо това.
- `email` трябва да бъде уникален в целия FastComments.com. Ако това е проблем, предлагаме да използвате SSO вместо това.
- Не можете да създавате повече tenant потребители от дефинираното под `maxTenantUsers` във вашия пакет.

Можем да създадем `TenantUser` по следния начин

[inline-code-attrs-start title = 'Пример за създаване на TenantUser с cURL'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за създаване на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
