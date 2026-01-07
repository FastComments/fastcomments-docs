[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя създаването на единичен SSO потребител.

Опитът да създадете двама потребители с едно и също ID ще доведе до грешка.

[inline-code-attrs-start title = 'Пример за създаване на SSOUser с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

В този пример указваме `groupIds` за контрол на достъпа, но това е незадължително.

[inline-code-attrs-start title = 'Структура на заявката за създаване на SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Бележка за интеграция

Данните, подадени от API, могат да бъдат презаписани просто чрез подаване на различен SSO User HMAC payload. Например, ако
зададете потребителско име чрез API, но след това подадете различно чрез SSO потока при зареждане на страницата, ние автоматично ще актуализираме
потребителското му име.

Ние няма да актуализираме параметрите на потребителя в този поток, освен ако изрично не ги посочите или ги зададете на null (не undefined).
