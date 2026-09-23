[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Опозивање гласа. Опција на коју је глас био дат враћа свој број, а гласач је слободан да гласа поново.

[inline-code-attrs-start title = 'PollVote брисање cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote брисање захтева структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote брисање одговора структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Анкета са ажурираним бројевима. **/
    poll: CommentPoll
}
[inline-code-end]

### Остале напомене

- Брисање истог гласа два пута даје `not-found` у другој прилици, а бројеви остају непромењени.
- Ако је анкета замењена од када је глас дат, глас се уклања али се бројеви не мењају, јер је замена започела од нуле.