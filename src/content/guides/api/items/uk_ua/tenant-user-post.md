[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Цей маршрут надає можливість додати одного `TenantUser`.

Створення `TenantUser` має такі обмеження:

- Потрібен `username`.
- Потрібен `email`.
- Поле `signUpDate` не може містити дату в майбутньому.
- Значення `locale` має бути в списку [Підтримувані локалі](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` має бути унікальним у межах всього FastComments.com. Якщо це проблема, ми радимо використовувати SSO.
- `email` має бути унікальним у межах всього FastComments.com. Якщо це проблема, ми радимо використовувати SSO.
- Ви не можете створити більше `TenantUser`, ніж визначено в `maxTenantUsers` у вашому пакеті. 

Ми можемо створити `TenantUser` таким чином

[inline-code-attrs-start title = 'Приклад cURL для створення TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту створення TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при створенні TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Включено у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Включено у разі невдачі. **/
    reason?: string
    tenantUser?: TenantUser; // Ми повертаємо повністю створеного TenantUser у разі успіху.
}
[inline-code-end]

---