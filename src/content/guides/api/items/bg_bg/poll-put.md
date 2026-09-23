[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикачва анкета към съществуващ коментар или задава пълното състояние на вече съществуващата анкета.

Тялото е пълната анкета, а изпратените опции стават опциите на анкетата, в този ред. Всяка опция се съпоставя по нейното `id`:

- Опция, изпратена с `id` на съществуваща опция, запазва тази опция и нейните гласове. Нейният етикет и позиция се актуализират според изпратеното.
- Опция, изпратена без `id`, се добавя, без гласове.
- Съществуваща опция, която пропуснете, се премахва, заедно с гласовете, дадени за нея. `totalVotes` намалява със същото количество.

Така че за да добавите опция, изпратете текущите опции с техните id‑та плюс новата без id. За да премахнете опция, изпратете списъка без нея. Id‑тата на опциите се намират в анкетата, върната от `GET /api/v1/polls/:commentId`.

Изпращането без никакви id‑та заменя всяка опция и изтрива всички гласове, вече дадени за анкетата. Ако анкетата има гласове, това изисква `replaceVotes=true`, а без него API‑ът отговаря с `replace-votes-required`.

Останалите полета също се заменят: пропускането на `closesAt`, `privacy` или `requireVoteToSeeResults` ги връща към стойността по подразбиране. За да промените едно поле и да оставите останалите непроменени, използвайте `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Пример за cURL PUT заявка за анкета'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявка за PUT на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Необходимо, за да се запазят нито едно от съществуващите id‑та на опциите, когато анкетата има гласове, тъй като това ги изтрива всички. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id‑то на съществуваща опция, за да се запази тя и нейните гласове. Пропуснете, за да добавите нова опция. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Пълният, подреден списък. Оставените извън него съществуващи опции се премахват заедно с техните гласове. **/
    options: PollPutOption[]
    /** Трябва да бъде в бъдещето, когато коментарът все още няма анкета. Пропуснете за анкета, която остава отворена. **/
    closesAt?: string | null
    /** 0 анонимно (по подразбиране), 1 администратори и модератори, 2 всички. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за PUT на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Включено при неуспех. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- `id`, което не съществува в анкетата, или едно и също `id`, дадено два пъти, води до грешка `poll-invalid`. Коментар без анкета все още няма id‑та на опциите, затова всяка изпратена към него опция трябва да пропусне `id`.
- Поверителността на анкетата може да се стеснява, но не и разширява, след като има гласове.
- Този API спазва настройките на вашия сайт. Ако анкетите не са активирани за сайта или страницата, се връща грешка `polls-disabled`.
- Заключен коментар не може да има променяна анкета и връща грешка `locked`.
- Свързаните уиджети се актуализират в реално време, така че зрителите виждат новата анкета без презареждане.