[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикачење анкете постојећем коментару, или поставља пун статус анкете која већ постоји.

Тело је комплетна анкета, а опције које пошаљете постају опције анкете, у том редоследу. Свака опција се упоређује помоћу њеног `id`:

- Опција послата са `id` постојеће опције задржава ту опцију и њене гласове. Њена ознака и позиција се ажурирају према ономе што сте послали.
- Опција послата без `id` се додаје, без гласова.
- Постојећа опција коју изоставите се уклања, заједно са гласовима датим за њу. `totalVotes` се смањује за исти износ.

Дакле, да бисте додали опцију, пошаљите тренутне опције са њиховим id-јевима и нову без id-а. Да бисте уклонили опцију, пошаљите листу без ње. id-еви опција се налазе у анкети која се враћа из `GET /api/v1/polls/:commentId`.

Слање без икаквих id-ева замењује сваку опцију и брише сваки глас који је већ дат у анкети. Ако анкета има гласове, ово захтева `replaceVotes=true`, а без тога API одговара са `replace-votes-required`.

Остала поља се такође замењују: изостављање `closesAt`, `privacy` или `requireVoteToSeeResults` враћа их на подразумевану вредност. Да бисте променили само једно поље и оставили остала нетакнута, користите `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Other Notes

- `id` који није у анкети, или исти `id` наведен два пута, резултује грешком `poll-invalid`. Коментар без анкете још увек нема id-еве опција, па свака послата опција мора изоставити `id`.
- Приватност анкете може бити сужена, али не може бити проширена након што има гласова.
- Овај API поштује подешавања вашег сајта. Ако анкете нису омогућене за сајт или страницу, резултује грешком `polls-disabled`.
- Закључани коментар не може имати промену анкете, и резултује грешком `locked`.
- Повезани виџети се ажурирају уживо, тако да гледаоци виде нову анкету без поновног учитавања.