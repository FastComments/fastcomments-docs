[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить одного `TenantUser`.

При создании `TenantUser` действуют следующие ограничения:

- `username` обязателен.
- `email` обязателен.
- `signUpDate` не может быть в будущем.
- `locale` должен находиться в списке [Поддерживаемые локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` должен быть уникален для всего FastComments.com. Если это является проблемой, мы рекомендуем вместо этого использовать SSO.
- `email` должен быть уникален для всего FastComments.com. Если это является проблемой, мы рекомендуем вместо этого использовать SSO.
- Вы не можете создать больше tenant users, чем определено в `maxTenantUsers` в вашем пакете. 

Мы можем создать `TenantUser` следующим образом

[inline-code-attrs-start title = 'Пример cURL-запроса для создания TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для создания TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при создании TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Включается при ошибке. **/
    reason?: string
    tenantUser?: TenantUser; // Мы возвращаем полностью созданного TenantUser при успехе.
}
[inline-code-end]