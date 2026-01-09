[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Этот маршрут обеспечивает создание одного SSO-пользователя.

Попытка создать двух пользователей с одинаковым ID приведёт к ошибке.

[inline-code-attrs-start title = 'Пример cURL-запроса создания SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

В этом примере мы указываем `groupIds` для управления доступом, но это необязательно.

[inline-code-attrs-start title = 'Структура запроса создания SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа создания SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Включается при ошибке. **/
    reason?: string
    user?: SSOUser; // Возвращаем созданного пользователя при успешном выполнении.
}
[inline-code-end]

#### Примечание по интеграции

Данные, переданные через API, могут быть переопределены просто путем передачи другого SSO User HMAC payload. Например, если вы зададите username через API, но затем передадите другой через SSO-процесс при загрузке страницы, мы автоматически обновим их username.

Мы не будем обновлять параметры пользователя в этом потоке, если вы явно не укажете их или не установите их в null (не undefined).