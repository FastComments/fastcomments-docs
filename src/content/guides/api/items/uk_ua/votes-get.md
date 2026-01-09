[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Голоси потрібно отримувати за допомогою `urlId`.

### Типи голосів

Існує три типи голосів:

- Авторизовані голоси, які застосовуються до відповідного коментаря. Ви можете створити їх через цей API.
- Авторизовані голоси, які перебувають у **очікуванні** верифікації, і тому ще не застосовані до коментаря. Вони створюються, коли користувач використовує на FastComments.com механізм *увійти, щоб проголосувати*.
- Анонімні голоси, які застосовуються до відповідного коментаря. Вони створюються разом з анонімним коментуванням.

Щоб уникнути плутанини, вони повертаються в окремих списках у API.

[inline-code-attrs-start title = 'Приклад cURL-запиту для голосів'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту голосів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді голосів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Включено у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Включено у разі помилки. **/
    reason?: string
    /** Авторизовані, підтверджені голоси, застосовані до відповідних коментарів. **/
    appliedAuthorizedVotes: Vote[]
    /** Анонімні голоси, застосовані до відповідних коментарів. **/
    appliedAnonymousVotes: Vote[]
    /** Голоси, які очікують верифікації і ще не застосовані до відповідних коментарів. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Примітки щодо анонімних голосів

Зауважте, що анонімні голоси, створені через цей API, з’являться у списку `appliedAuthorizedVotes`. Вони вважаються авторизованими, оскільки були створені через API з використанням API key.

Структура `appliedAnonymousVotes` призначена для голосів, створених без електронної пошти, API key тощо.