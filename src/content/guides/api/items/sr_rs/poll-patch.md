[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Уређује анкету без утицаја на њене гласове. Користите ово за исправку правописне грешке у питању или опцији, за затварање или поновно отварање анкете, или за промену ко може видети ко је гласао.

Опције се адресирају помоћу њихових `id`, а `PATCH` преименује оне које наведете. Да бисте додали, уклонили или променили редослед опција, пошаљите комплетан списак опција на `PUT /api/v1/polls/:commentId`: опције које пошаљете са њиховим id-јима задржавају и своје гласове.

Сва поље је опционо, али мора бити наведено најмање једно.

[inline-code-attrs-start title = 'Пример cURL захтева за измену анкете'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за тренутно затварање анкете'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за измену анкете'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора на измену анкете'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Остале напомене

- Додељивање id опције која не постоји у анкети доводи до грешке `poll-invalid` уместо тихог неурађивања.
- Ознаке морају остати јединствене унутар анкете, укључујући и опције које не мењате.
- За разлику од креирања анкете, `closesAt` може бити у прошлости – тако се анкета одмах затвара.
- Приватност анкете се може сузити, али не и проширити након што има гласова.
- Закључан коментар не може имати промену анкете и резултује грешком `locked`.