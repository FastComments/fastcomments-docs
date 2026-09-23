[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Редактира анкета без да пречи на гласовете й. Използвайте това, за да поправите правописна грешка в въпроса или в опция, за да затворите или отворите отново анкетата, или за да промените кой може да вижда кой е гласувал.

Опциите се идентифицират чрез техния `id`, а `PATCH` преименува тези, които посочите. За да добавите, премахнете или пренаредите опции, изпратете пълния списък с опции към `PUT /api/v1/polls/:commentId`: опциите, които изпратите с техните идентификатори, запазват гласовете си.

Всяко поле е незадължително, но трябва да бъде предоставено поне едно.

[inline-code-attrs-start title = 'Пример cURL за актуализиране на анкета'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL за незабавно затваряне на анкета'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за актуализиране на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на отговор за актуализиране на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Други бележки

- Назоваване на идентификатор на опция, който не съществува в анкетата, води до грешка `poll-invalid`, вместо тихо да не прави нищо.
- Етикетите трябва да останат уникални в анкетата, като се броят и опциите, които не променяте.
- За разлика от създаването на анкета, `closesAt` тук може да бъде в миналото – това е начинът да затворите анкета незабавно.
- Поверителността на анкетата може да се стеснява, но не и да се разширява след като има гласове.
- Заключен коментар не може да има променена анкета и връща грешка `locked`.