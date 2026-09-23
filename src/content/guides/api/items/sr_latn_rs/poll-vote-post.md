[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Zapisuje glas na anketi.

Glasač može imati najviše jedan glas po anketi. Ponovnim pozivanjem za istog glasača njegov glas se premesti na novu opciju umesto da se doda drugi, a glasanje za opciju koju je već izabrao ne radi ništa.

Odgovor uključuje anketu, tako da dobijate ažurirane brojeve bez drugog zahteva.

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primer cURL zahteva za anonimno kreiranje PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** Jedan od userId ili anonUserId je obavezan. **/
    userId?: string
    anonUserId?: string
    /** IP krajnjeg korisnika, korišćen za anonimno ograničenje brzine. Podrazumevano je IP pozivaoca. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Uključeno u slučaju greške. **/
    reason?: string
    pollVote: PollVote
    /** Anketa sa ažuriranim brojevima. **/
    poll: CommentPoll
}
[inline-code-end]

### Anonimni glasovi

Postavite `anonUserId` umesto `userId` da zabeležite glas za nekoga ko nije prijavljen. Taj ID ne mora da odgovara nekom korisniku - on samo identifikuje sesiju, tako da ista osoba ne bude brojana dva puta.

Anonimno glasanje mora biti omogućeno za vaš sajt. Ako je glasanje ograničeno na prijavljene korisnike, glas sa samo `anonUserId` će propasti sa `poll-login-required`.

Anonimni glasovi su takođe ograničeni po IP-u po anketi, kako bi se sprečilo da jedna osoba napuni anketu brisanjem sesije. Pošaljite `ip` krajnjeg korisnika da se ograničenje primeni na njega, a ne na vaš server.

### Ostale napomene

- `userId` mora biti korisnik koji postoji na vašem sajtu. Glasovi za korisnika koji pripada drugom sajtu se odbacuju.
- Glasanje na zatvorenoj anketi propada sa `poll-closed`.
- Ovaj API ažurira brojeve na anketi i šalje ih u povezane vidžete u realnom vremenu.

---