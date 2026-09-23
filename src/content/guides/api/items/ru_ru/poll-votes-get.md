[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Список отдельных голосов, лежащих в основе подсчётов одного опроса, в порядке от старейшего к новейшему. Один кредит за каждые 100 возвращённых голосов.

Опрос принадлежит комментарию, поэтому голоса читаются по одному опросу за раз, и параметр `commentId` обязателен. Можно уточнить запрос с помощью `voterId`, чтобы проверить, как проголосовал конкретный пользователь, или с помощью `optionId`, чтобы получить список всех, кто выбрал определённый вариант.

За один вызов возвращается не более 1000 голосов. Используйте `skip` для постраничного получения остальных.

Настройка `privacy` опроса учитывается: голоса в анонимном опросе нельзя прочитать, и запрос завершится ошибкой `poll-anonymous`. Подробнее см. структуру `PollVote`.

[inline-code-attrs-start title = 'Пример cURL запроса PollVotes Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Подсчёт голосов по варианту

Вам не нужно суммировать их, чтобы получить результаты — опрос уже содержит собственные подсчёты. Вместо этого прочитайте опрос с помощью `GET /api/v1/polls/:commentId`, а этот API используйте, когда необходимо узнать, кто проголосовал.

### Каждый опрос на странице

Список голосов по всей странице отсутствует. Чтобы собрать отчёт по всей странице, получите её комментарии с помощью `GET /api/v1/comments`, который возвращает опрос каждого комментария и его подсчёты, после чего можно прочитать голоса для интересующих вас опросов.

---