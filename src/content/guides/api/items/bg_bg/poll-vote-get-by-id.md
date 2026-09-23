---
[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Чете един глас от анкета по неговото ID.

Глас от анонимна анкета не може да бъде прочетен и заявката се проваля с `poll-anonymous`. Вижте структурата `PollVote` за това как се прилага настройката `privacy` на анкетата.

[inline-code-attrs-start title = 'PollVote Get cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get заявка структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get структура на отговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** Включено при неуспех. **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]

---