[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикачује анкету на постојећи коментар, или поставља комплетно стање анкете коју већ има.

Тело захтева је комплетна анкета, а опције које пошаљете постају опције анкете, тим редоследом. Свака опција
се подудара према свом `id`:

- Опција послата са `id` постојеће опције задржава ту опцију и њене гласове. Њена ознака и позиција
  се ажурирају на оно што сте послали.
- Опција послата без `id` се додаје, без гласова.
- Постојећа опција коју изоставите се уклања, заједно са гласовима датим на њу. `totalVotes` се смањује за исти
  износ.

Дакле, да бисте додали опцију, пошаљите тренутне опције са њиховим id-јевима плус нову без id-ја. Да бисте уклонили
једну, пошаљите листу без ње. Id-јеви опција се налазе на анкети враћеној преко `GET /api/v1/polls/:commentId`.

Слање без иједног id-ја замењује све опције и брише све гласове већ дате на анкети. Ако анкета има
гласове, ово захтева `replaceVotes=true`, а без тога API одговара са `replace-votes-required`.

Остала поља се такође замењују: изостављање `closesAt`, `privacy` или `requireVoteToSeeResults` их враћа на
подразумеване вредности. Да бисте променили само једно поље и оставили остало недирнуто, користите `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Пример cURL захтева за Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура Poll Put захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Обавезно када се не задржава ниједан постојећи id опције, а анкета има гласове, пошто то брише све гласове. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id постојеће опције, да би се задржала заједно са својим гласовима. Изоставите за додавање нове опције. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Комплетна, уређена листа. Изостављене постојеће опције се уклањају заједно са својим гласовима. **/
    options: PollPutOption[]
    /** Мора бити у будућности када коментар још нема анкету. Изоставите за анкету која остаје отворена. **/
    closesAt?: string | null
    /** 0 анонимно (подразумевано), 1 админи и модератори, 2 сви. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура Poll Put одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Укључено у случају неуспеха. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Остале напомене

- `id` који није на анкети, или исти `id` дат два пута, неуспева са `poll-invalid`. Коментар без
  анкете нема id-јеве опција, тако да свака послата опција мора бити без `id`.
- Приватност анкете може се сузити, али не и проширити када има гласове.
- Овај API поштује подешавања вашег сајта. Ако анкете нису омогућене за сајт или страницу, неуспева са
  `polls-disabled`.
- Закључан коментар не може имати промењену анкету и неуспева са `locked`.
- Повезани виџети се ажурирају уживо, тако да посматрачи виде нову анкету без освежавања странице.