[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Голоса должны извлекаться по `urlId`.

### Типы голосов

Существует три типа голосов:

- Авторизованные голоса, которые применяются к соответствующему комментарию. Вы можете создавать их через этот API.
- Авторизованные голоса, которые **в ожидании подтверждения**, и поэтому ещё не применены к комментарию. Они создаются, когда пользователь использует FastComments.com *вход для голосования*.
- Анонимные голоса, которые применяются к соответствующему комментарию. Они создаются вместе с анонимными комментариями.

Они возвращаются в отдельных списках в API для уменьшения путаницы.

[inline-code-attrs-start title = 'Пример cURL-запроса голосов'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса голосов'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа голосов'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Включается при ошибке. **/
    reason?: string
    /** Авторизованные, подтверждённые голоса, применённые к соответствующим комментариям. **/
    appliedAuthorizedVotes: Vote[]
    /** Анонимные голоса, применённые к соответствующим комментариям. **/
    appliedAnonymousVotes: Vote[]
    /** Голоса, ожидающие проверки, ещё не применённые к соответствующим комментариям. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Примечания по анонимным голосам

Обратите внимание, что анонимные голоса, созданные через этот API, будут появляться в списке `appliedAuthorizedVotes`. Они считаются авторизованными, поскольку были созданы через API с использованием API-ключа.

Структура `appliedAnonymousVotes` предназначена для голосов, созданных без электронной почты, API-ключа и т.д.

---