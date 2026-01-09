[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Голоса необходимо получать по `urlId`.

### Типы голосов

Существует три типа голосов:

- Авторизованные голоса, которые применяются к соответствующему комментарию. Вы можете создавать их через этот API.
- Авторизованные голоса, которые находятся в процессе верификации, и поэтому ещё не применены к комментарию. Они создаются, когда пользователь использует механизм FastComments.com *вход для голосования*.
- Анонимные голоса, которые применяются к соответствующему комментарию. Они создаются вместе с анонимными комментариями.

Они возвращаются в отдельных списках в API, чтобы уменьшить путаницу.

[inline-code-attrs-start title = 'Пример cURL-запроса для голосов'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Включено при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Включено при ошибке. **/
    reason?: string
    /** Авторизованные, проверенные голоса, применённые к соответствующим комментариям. **/
    appliedAuthorizedVotes: Vote[]
    /** Анонимные голоса, применённые к соответствующим комментариям. **/
    appliedAnonymousVotes: Vote[]
    /** Голоса, ожидающие верификации, ещё не применённые к соответствующим комментариям. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Примечания по анонимным голосам

Обратите внимание, что анонимные голоса, созданные через этот API, появятся в списке `appliedAuthorizedVotes`. Они считаются авторизованными, так как были созданы через API с использованием API-ключа.

Структура `appliedAnonymousVotes` предназначена для голосов, созданных без электронной почты, API-ключа и т.д.

---