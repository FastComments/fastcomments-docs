[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Чита један глас у анкети према његовој ID.

Глас у анонимној анкети не може бити прочитан, а захтев не успева са `poll-anonymous`. Погледајте структуру `PollVote` да бисте видели како се примењује подешавање `privacy` анкете.

[inline-code-attrs-start title = 'PollVote Get cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get захтевна структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Get одговорна структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** Укључено при неуспеху. **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]