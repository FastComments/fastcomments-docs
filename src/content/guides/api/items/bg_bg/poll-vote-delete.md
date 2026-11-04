[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Оттегля глас. Опцията, за която е подаден, си връща броя, а гласувалият може отново да гласува.

[inline-code-attrs-start title = 'Пример за изтриване на PollVote с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за изтриване на PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за изтриване на PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Други бележки

- При повторно изтриване на същия глас вторият отговор е `not-found`, а броячите не се променят.
- Ако анкетата е била заменена след подаването на гласа, гласът се премахва, но без промяна на броячите, тъй като
  заменящата анкета е започнала от нула.