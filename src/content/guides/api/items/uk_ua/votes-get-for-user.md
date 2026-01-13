---
[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Дозволяє отримати голоси, залишені користувачем для певного `urlId`. Потребує `userId`, який може бути будь-яким користувачем FastComments.com або `SSO User`.

Це корисно, якщо ви хочете показати, чи голосував користувач за коментар. Коли ви отримуєте коментарі, просто викликайте цей API одночасно для цього користувача з тією ж `urlId`.

Якщо ви використовуєте анонімне голосування, передавайте натомість `anonUserId`.

[inline-code-attrs-start title = 'Приклад cURL: Голоси для користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL: Голоси для анонімного користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Зверніть увагу, що анонімні голоси з'являться в списку `appliedAuthorizedVotes`. Вони вважаються авторизованими, оскільки були створені через API з використанням API-ключа.

[inline-code-attrs-start title = 'Структура запиту: Голоси для користувача'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді: Голоси для користувача'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Включається у разі помилки. **/
    reason?: string
    /** Авторизовані, перевірені голоси, застосовані до відповідних коментарів. **/
    appliedAuthorizedVotes: Vote[]
    /** Голоси, що очікують перевірки, ще не застосовані до відповідних коментарів. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---