[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Бележи глас за анкету.

Гласач може имати највише један глас по анкети. Поновним позивом за истог гласача премешта се њихов глас на нову опцију уместо додавања другог, а гласање за опцију коју су већ изабрали не утиче.

Одговор укључује анкету, тако да добијате ажуриране бројеве без другог захтева.

[inline-code-attrs-start title = 'Пример cURL захтева за креирање PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за анонимно креирање PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за креирање PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** Један од userId или anonUserId је обавезан. **/
    userId?: string
    anonUserId?: string
    /** IP крајњег корисника, користи се за анонимно ограничење брзине. Подразумевано је IP позиваоца. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за креирање PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Укључено у случају неуспеха. **/
    reason?: string
    pollVote: PollVote
    /** Анкета са ажурираним бројевима. **/
    poll: CommentPoll
}
[inline-code-end]

### Анонимни гласови

Поставите `anonUserId` уместо `userId` да бисте записали глас за некога ко није пријављен. Тај идентификатор не мора да одговара кориснику нигде – он само идентификује сесију, тако да се исти особа не броји два пута.

Анонимно гласање мора бити омогућено за ваш сајт. Ако је гласање ограничено на пријављене кориснике, глас са само `anonUserId` не успева са `poll-login-required`.

Анонимни гласови су такође ограничени по IP-у по анкети, како би се спречило да једна особа попуни анкету брисањем сесије. Пошаљите IP крајњег корисника `ip` како би ограничење важило за њих, а не за ваш сервер.

### Остале напомене

- `userId` мора да буде корисник који постоји на вашем сајту. Гласови за корисника који припада другом сајту се одбацују.
- Гласање у затвореној анкети не успева са `poll-closed`.
- Овај API ажурира бројеве у анкети и у реалном времену их прослеђује повезаним виџетима.