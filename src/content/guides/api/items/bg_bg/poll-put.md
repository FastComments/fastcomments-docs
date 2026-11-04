[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикачва анкета към съществуващ коментар или задава пълното състояние на анкетата, която вече има.

Тялото е пълната анкета, а опциите, които изпращате, стават опциите на анкетата, в този ред. Всяка опция
се съпоставя по нейния `id`:

- Опция, изпратена с `id` на съществуваща опция, запазва тази опция и нейните гласове. Етикетът ѝ и позицията ѝ
  се обновяват според изпратеното.
- Опция, изпратена без `id`, се добавя без гласове.
- Съществуваща опция, която пропуснете, се премахва заедно с гласуванията за нея. `totalVotes` намалява със
  същото количество.

Така че, за да добавите опция, изпратете текущите опции с техните id-та плюс новата без id. За да премахнете
опция, изпратете списъка без нея. Id-тата на опциите са в анкетата, върната от `GET /api/v1/polls/:commentId`.

Изпращането без никакви id-та замества всички опции и изтрива всички вече подадени гласове в анкетата. Ако
анкетата има гласове, това изисква `replaceVotes=true`, а без него API-то отговаря с `replace-votes-required`.

Останалите полета също се заместват: пропускането на `closesAt`, `privacy` или `requireVoteToSeeResults` го
връща към стойността по подразбиране. За да промените само едно поле и да оставите останалите непокътнати,
използвайте `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Пример за Poll Put с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Кой release да пуснем първо?",
	"options": [
		{"id": "existing-option-id", "label": "Release с поправките"},
		{"label": "Release с новите функции"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Задължително, когато не се запазва нито едно съществуващо id на опция и анкетата има гласове, тъй като това ги изтрива всички. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id на съществуваща опция, за да се запази тя и нейните гласове. Пропуснете, за да добавите нова опция. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Пълният, подреден списък. Пропуснатите съществуващи опции се премахват заедно с гласуванията им. **/
    options: PollPutOption[]
    /** Трябва да е в бъдещето, когато коментарът още няма анкета. Пропуснете за анкета, която остава отворена. **/
    closesAt?: string | null
    /** 0 анонимни (по подразбиране), 1 администратори и модератори, 2 всички. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Включва се при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Включва се при неуспех. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Други бележки

- `id`, което не е в анкетата, или едно и също `id`, подадено два пъти, води до неуспех с `poll-invalid`. Коментар без
  анкета няма id-та на опции, така че всяка изпратена опция към него трябва да пропусне `id`.
- Поверителността на анкетата може да се стесни, но не и да се разшири, след като има гласове.
- Това API спазва настройките на вашия сайт. Ако анкетите не са разрешени за сайта или страницата, заявката се
  проваля с `polls-disabled`.
- Заключен коментар не може да има променена анкета и се проваля с `locked`.
- Свързаните уиджети се обновяват на живо, така че зрителите виждат новата анкета без презареждане.