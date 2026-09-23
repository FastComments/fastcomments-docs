[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Изброява индивидуалните гласове зад брояча на една анкета, най-старите първи. Един кредит за всеки 100 върнати гласа.

Анкета принадлежи на коментар, затова гласовете се четат по една анкета наведнъж и `commentId` е задължително. Ограничете още повече с `voterId`, за да проверите как е гласувал конкретен потребител, или с `optionId`, за да изброите всички, които са избрали дадена опция.

Най‑много 1000 гласа се връщат за едно повикване. Използвайте `skip`, за да преминавате към следващите.

Настройката за `privacy` на анкетата се спазва: гласовете в анонимна анкета не могат да се четат и заявката се проваля с `poll-anonymous`. Вижте структурата `PollVote` за подробности.

[inline-code-attrs-start title = 'PollVotes Get cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get заявка структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get отговор структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Броене на гласове по опция

Не е необходимо да сумирате тези гласове, за да получите резултатите – анкетата съдържа собствените си броячи. Прочетете анкетата с `GET /api/v1/polls/:commentId` вместо това и използвайте това API, когато трябва да знаете кой е гласувал.

### Всяка анкета на страница

Няма списък с гласове за цялата страница. За да отчетете цяла страница, извлечете нейните коментари с `GET /api/v1/comments`, което връща анкетата и брояча за всеки коментар, след което прочетете гласовете за анкетите, които ви интересуват.

---