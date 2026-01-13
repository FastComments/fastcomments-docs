[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Цей маршрут дозволяє створити одного SSO-користувача.

Спроба створити двох користувачів з однаковим ID призведе до помилки.

[inline-code-attrs-start title = 'Приклад cURL створення SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

У цьому прикладі ми вказуємо `groupIds` для контролю доступу, але це необов'язково.

[inline-code-attrs-start title = 'Структура запиту створення SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді при створенні SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Включається у разі помилки. **/
    reason?: string
    user?: SSOUser; // Ми повертаємо створеного користувача у разі успіху.
}
[inline-code-end]

#### Примітка щодо інтеграції

Дані, передані через API, можна перевизначити простим передаванням іншого SSO User HMAC payload. Наприклад, якщо
ви задасте username через API, але потім передасте інший через SSO-процес при завантаженні сторінки, ми автоматично оновимо
їхній username.

Ми не будемо оновлювати параметри користувача в цьому процесі, якщо ви явно не вкажете їх або не встановите їх у null (не undefined).

---