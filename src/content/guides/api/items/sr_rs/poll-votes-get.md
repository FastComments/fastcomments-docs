[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Листа појединачних гласова иза бројања једне анкете, најстарије прво. Један кредит по 100 вратених гласова.

Анкета припада коментару, па се гласови читају по једној анкети у исто време и `commentId` је обавезан. Додатно сузите претрагу са `voterId` да проверите како је једна особа гласала, или са `optionId` да наведете све који су изабрали одређену опцију.

Највише 1000 гласова се враћа по позиву. Користите `skip` за прелазак на следећи скуп.

Подешавање `privacy` анкете се поштује: гласови на анонимној анкети се не могу прочитати, а захтев ће пропасти са `poll-anonymous`. Погледајте структуру `PollVote` за детаље.

[inline-code-attrs-start title = 'PollVotes Get cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get захтевна структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get одговорна структура'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Бројање гласова по опцији

Не морате их сабирати да бисте добили резултате – анкета сама садржи своје бројеве. Уместо тога прочитајте анкету помоћу `GET /api/v1/polls/:commentId`, и користите овај API када треба да знате ко је гласао.

### Свака анкета на страници

Не постоји листа гласова за целу страницу. Да бисте извештавали о целој страници, преузмите њене коментаре помоћу `GET /api/v1/comments`, што враћа анкету сваког коментара и њене бројеве, а затим прочитајте гласове за анкете које вас интересују.