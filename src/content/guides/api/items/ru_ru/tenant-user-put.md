[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность заменить одного `TenantUser`.

При замене `TenantUser` действуют следующие ограничения:

- `signUpDate` не может быть в будущем.
- `locale` должен находиться в списке [Поддерживаемые локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` должен быть уникален для всего FastComments.com. Если это проблема, мы рекомендуем вместо этого использовать SSO.
- `email` должен быть уникален для всего FastComments.com. Если это проблема, мы рекомендуем вместо этого использовать SSO.
- Вы не можете обновлять `tenantId` пользователя.

Мы можем создать `TenantUser` следующим образом

[inline-code-attrs-start title = 'Пример cURL для замены TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для замены TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Когда email или username изменяются, вы можете установить это в true, чтобы также обновить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для замены TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]

---