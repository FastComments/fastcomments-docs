[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Редагує опитування, не порушуючи його голосів. Використовуйте це, щоб виправити помилку в питанні або варіанті, щоб закрити або
відкрити опитування, або змінити, хто може бачити, хто проголосував.

Опції ідентифікуються за їх `id`, і `PATCH` перейменовує ті, які ви вказали. Щоб додати, видалити або змінити порядок
опцій, надішліть повний список опцій до `PUT /api/v1/polls/:commentId`: опції, які ви надсилаєте з їх id, зберігають
їх голоси.

Кожне поле є необов’язковим, але має бути вказано хоча б одне.

[inline-code-attrs-start title = 'Приклад cURL для оновлення опитування'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL для негайного закриття опитування'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту оновлення опитування'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді оновлення опитування'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Інші нотатки

- Назва id опції, якої немає в опитуванні, призводить до помилки `poll-invalid`, а не просто нічого не робить.
- Мітки повинні залишатися унікальними в межах опитування, враховуючи опції, які ви не змінюєте.
- На відміну від створення опитування, `closesAt` може бути в минулому — це спосіб негайно закрити опитування.
- Приватність опитування можна звузити, але не розширити після того, як у ньому є голоси.
- Заблокований коментар не може мати змінене опитування і повертає помилку `locked`.

---