[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Zapiše glas na anketi.

Glasovalec ima največ en glas na anketo. Ponovno klicanje te metode za istega glasovalca premakne njihov glas na novo možnost, namesto da bi dodalo drugi glas, in glasovanje za možnost, ki so jo že izbrali, ne naredi ničesar.

Odgovor vključuje anketo, tako da dobite posodobljene štetje brez druge zahteve.

[inline-code-attrs-start title = 'Primer cURL za ustvarjanje PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primer cURL za anonimno ustvarjanje PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteve za ustvarjanje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Anonimni glasovi

Nastavite `anonUserId` namesto `userId`, da zabeležite glas za nekoga, ki ni prijavljen. Ta ID ne mora ustrezati uporabniku nikjer – le identificira sejo, tako da ista oseba ne bo šteta dvakrat.

Anonimno glasovanje mora biti omogočeno za vaše spletno mesto. Če je glasovanje omejeno na prijavljene uporabnike, glas z le `anonUserId` ne uspe in vrne `poll-login-required`.

Anonimni glasovi so tudi omejeni po hitrosti na IP za vsako anketo, da se prepreči, da bi ena oseba napolnila anketo z brisanjem svoje seje. Pošljite končnemu uporabniku `ip`, da se omejitev uporabi zanj, namesto na vaš strežnik.

### Druge opombe

- `userId` mora biti uporabnik, ki obstaja na vašem spletnem mestu. Glasovi za uporabnika, ki pripada drugemu spletnemu mestu, so zavrnjeni.
- Glasovanje na zaprto anketo ne uspe in vrne `poll-closed`.
- Ta API posodablja štetje na anketi in jih v živo pošilja povezanih pripomočkov.