[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить одного `TenantUser`.

Создание `TenantUser` имеет следующие ограничения:

- `username` обязателен.
- `email` обязателен.
- `signUpDate` не может быть в будущем.
- `locale` должен быть в списке [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` должен быть уникальным на FastComments.com. Если это проблема, мы рекомендуем использовать SSO.
- `email` должен быть уникальным на FastComments.com. Если это проблема, мы рекомендуем использовать SSO.
- Вы не можете создать больше `tenant users`, чем определено в `maxTenantUsers` в вашем пакете. 

Мы можем создать `TenantUser` следующим образом

[inline-code-attrs-start title = 'Пример cURL запроса для создания TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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
    /** Включается в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Включается в случае ошибки. **/
    reason?: string
    tenantUser?: TenantUser; // При успехе возвращается полностью созданный TenantUser.
}
[inline-code-end]