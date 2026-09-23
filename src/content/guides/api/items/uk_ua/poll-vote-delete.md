[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Відкликає голос. Опція, на яку був відданий голос, повертає свій лічильник, і виборець може проголосувати знову.

[inline-code-attrs-start title = 'Приклад cURL запиту для видалення PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту видалення PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на видалення PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Інші примітки

- Видалення того самого голосу двічі відповідає `not-found` під час другого запиту, і лічильники залишаються без змін.
- Якщо опитування було замінено після того, як був відданий голос, голос видаляється, але лічильники не змінюються, оскільки заміна починається з нуля.