[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет обновить одного `TenantUser`.

При обновлении `TenantUser` действуют следующие ограничения:

- Значение `signUpDate` не может быть в будущем.
- Значение `locale` должен быть в списке [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Значение `username` должно быть уникальным для всего FastComments.com. Если это проблема, мы рекомендуем использовать SSO.
- Значение `email` должно быть уникальным для всего FastComments.com. Если это проблема, мы рекомендуем использовать SSO.
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
    /** Если изменяется email или username, вы можете установить это в true, чтобы также обновить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа обновления TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Присутствует при ошибке. **/
    reason?: string
}
[inline-code-end]