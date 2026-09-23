[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Перелічує окремі голоси, що стоять за підрахунками одного опитування, у хронологічному порядку (спочатку найстаріші). Один кредит за кожні 100 повернутих голосів.

Опитування належить коментарю, тому голоси читаються по одному опитуванню за раз, і `commentId` є обов’язковим. Додатково можна уточнити за допомогою `voterId`, щоб перевірити, як проголосувала конкретна особа, або за допомогою `optionId`, щоб отримати список усіх, хто вибрав певний варіант.

За один виклик повертається максимум 1000 голосів. Використовуйте `skip`, щоб посторінково отримувати більше.

Налаштування `privacy` опитування дотримується: голоси в анонімному опитуванні не можуть бути прочитані, і запит завершується помилкою `poll-anonymous`. Дивіться структуру `PollVote` для деталей.

[inline-code-attrs-start title = 'Приклад cURL запиту PollVotes Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Підрахунок голосів за варіантом

Ви не повинні підсумовувати їх, щоб отримати результати — опитування містить власні підрахунки. Замість цього прочитайте опитування за допомогою `GET /api/v1/polls/:commentId`, і використовуйте цей API, коли потрібно знати, хто проголосував.

### Кожне опитування на сторінці

Списку голосів по всій сторінці не існує. Щоб отримати звіт за всю сторінку, отримайте її коментарі за допомогою `GET /api/v1/comments`, який повертає опитування кожного коментаря та їх підрахунки, а потім прочитайте голоси для опитувань, які вас цікавлять.

---