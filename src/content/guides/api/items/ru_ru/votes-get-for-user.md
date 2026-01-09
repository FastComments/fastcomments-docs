[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Позволяет получить голоса, оставленные пользователем для указанного `urlId`. Принимает `userId`, который может относиться к пользователю FastComments.com или к `SSO User`.

Это полезно, если вы хотите показать, проголосовал ли пользователь за комментарий. При получении комментариев просто вызовите этот API одновременно для пользователя с тем же `urlId`.

Если вы используете анонимное голосование, то вместо этого передайте `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL запроса для голосов пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL запроса для анонимного пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Обратите внимание, что анонимные голоса будут отображаться в списке `appliedAuthorizedVotes`. Они считаются авторизованными, поскольку были созданы через API с использованием API key.

[inline-code-attrs-start title = 'Структура запроса для голосов пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для голосов пользователя'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Включается при ошибке. **/
    reason?: string
    /** Авторизованные, проверенные голоса, применённые к соответствующим комментариям. **/
    appliedAuthorizedVotes: Vote[]
    /** Голоса, ожидающие проверки, ещё не применённые к соответствующим комментариям. **/
    pendingVotes: Vote[]
}
[inline-code-end]