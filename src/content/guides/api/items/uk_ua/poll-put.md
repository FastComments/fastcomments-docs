[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикріплює опитування до існуючого коментаря, або встановлює повний стан опитування, яке вже існує.

Тіло є повним опитуванням, і передані вами варіанти стають варіантами опитування у зазначеному порядку. Кожен варіант співставляється за його `id`:

- Варіант, надісланий з `id` існуючого варіанту, зберігає цей варіант і його голоси. Його мітка та позиція оновлюються згідно з надісланим.
- Варіант, надісланий без `id`, додається без голосів.
- Існуючий варіант, який ви пропускаєте, видаляється разом з голосами, відданими за нього. `totalVotes` зменшується на ту ж саму кількість.

Отже, щоб додати варіант, надішліть поточні варіанти з їхніми id та новий без id. Щоб видалити варіант, надішліть список без нього. Id варіантів містяться в опитуванні, яке повертає `GET /api/v1/polls/:commentId`.

Надсилання без жодних id замінює всі варіанти та видаляє всі вже віддані голоси в опитуванні. Якщо в опитуванні є голоси, це вимагає `replaceVotes=true`, а без цього API відповідає `replace-votes-required`.

Інші поля також замінюються: пропуск `closesAt`, `privacy` або `requireVoteToSeeResults` скидає їх до значення за замовчуванням. Щоб змінити лише одне поле і залишити інші без змін, використовуйте `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Приклад cURL запиту Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Інші нотатки

- `id`, який відсутній в опитуванні, або той самий `id`, вказаний двічі, призводить до помилки `poll-invalid`. Коментар без опитування ще не має id варіантів, тому кожен варіант, який надсилається, повинен мати пропущений `id`.
- Приватність опитування можна обмежити, але не розширити після того, як у ньому є голоси.
- Цей API дотримується налаштувань вашого сайту. Якщо опитування не ввімкнені для сайту або сторінки, виникає помилка `polls-disabled`.
- Заблокований коментар не може мати змінене опитування, і повертає помилку `locked`.
- Підключені віджети оновлюються в режимі реального часу, тому глядачі бачать нове опитування без перезавантаження.