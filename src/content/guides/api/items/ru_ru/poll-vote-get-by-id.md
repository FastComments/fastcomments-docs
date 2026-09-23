[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Читает один голос опроса по его идентификатору.

Голос в анонимном опросе нельзя прочитать, и запрос завершится ошибкой `poll-anonymous`. См. структуру `PollVote`, чтобы понять, как применяется настройка `privacy` опроса.

[inline-code-attrs-start title = 'Пример cURL запроса PollVote Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PollVote Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PollVote Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]