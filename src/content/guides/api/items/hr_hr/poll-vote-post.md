[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Bilježi glas na anketi.

Glasatelj može imati najviše jedan glas po anketi. Ponovnim pozivanjem za istog glasatelja premješta se njihov glas na novu opciju umjesto dodavanja drugog, a glasanje za opciju koju su već odabrali ne čini ništa.

Odgovor uključuje anketu, tako da dobivate ažurirane brojeve bez drugog zahtjeva.

[inline-code-attrs-start title = 'PollVote Create cURL Primjer'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Anonimni PollVote Create cURL Primjer'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVote Create Struktura Zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVote Create Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Postavite `anonUserId` umjesto `userId` kako biste zabilježili glas za nekoga tko nije prijavljen. Taj ID ne mora odgovarati korisniku bilo gdje – samo identificira sesiju, tako da ista osoba nije brojana dvaput.

Anonimno glasanje mora biti omogućeno za vašu stranicu. Ako je glasanje ograničeno na prijavljene korisnike, glas s jednim `anonUserId` ne uspijeva s `poll-login-required`.

Anonimni glasovi također su ograničeni po IP-u po anketi, kako bi se spriječilo da jedna osoba napuni anketu brisanjem svoje sesije. Pošaljite IP krajnjeg korisnika `ip` kako bi se ograničenje primijenilo na njih, a ne na vaš poslužitelj.

### Ostale napomene

- `userId` mora biti korisnik koji postoji na vašoj stranici. Glasovi za korisnika koji pripada drugoj stranici se odbacuju.
- Glasanje u zatvorenoj anketi ne uspijeva s `poll-closed`.
- Ovaj API ažurira brojeve na anketi i u stvarnom vremenu ih šalje povezanih widgeta live.

---