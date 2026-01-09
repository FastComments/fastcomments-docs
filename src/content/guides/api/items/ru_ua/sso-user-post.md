[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Этот маршрут обеспечивает создание одного SSO-пользователя.

Попытка создать двух пользователей с одинаковым ID приведёт к ошибке.

[inline-code-attrs-start title = 'Пример cURL-запроса для создания SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

В этом примере мы указываем `groupIds` для контроля доступа, но это необязательно.

[inline-code-attrs-start title = 'Структура запроса создания SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при создании SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Включается при ошибке. **/
    reason?: string
    user?: SSOUser; // Мы возвращаем созданного пользователя в случае успеха.
}
[inline-code-end]

#### Примечание по интеграции

Данные, переданные через API, можно переопределить, просто передав другой SSO User HMAC payload. Например, если
вы задали username через API, но затем передаёте другой в SSO-потоке при загрузке страницы, мы автоматически обновим
его username.

Мы не будем обновлять параметры пользователя в этом процессе, если вы явно не укажете их или не установите в null (не undefined).