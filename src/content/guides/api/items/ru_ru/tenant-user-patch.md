[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет обновить одного `TenantUser`.

Обновление `TenantUser` имеет следующие ограничения:

- `signUpDate` не может быть в будущем.
- `locale` должен быть в списке [Поддерживаемые локали](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` должен быть уникальным на всём FastComments.com. Если это проблема, мы рекомендуем вместо этого использовать SSO.
- `email` должен быть уникальным на всём FastComments.com. Если это проблема, мы рекомендуем вместо этого использовать SSO.
- Вы не можете обновлять `tenantId` пользователя.

Мы можем создать `TenantUser` следующим образом

[inline-code-attrs-start title = 'Пример cURL-запроса обновления TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса обновления TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Если email или username изменяются, вы можете установить это в true, чтобы также обновить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа обновления TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Включается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Включается при неудаче. **/
    reason?: string
}
[inline-code-end]