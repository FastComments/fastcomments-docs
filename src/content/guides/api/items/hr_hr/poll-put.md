[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Prilaže anketu postojećem komentaru ili postavlja potpuno stanje ankete koju već ima.

Tijelo je potpuna anketa, a opcije koje pošaljete postaju opcije ankete, tim redoslijedom. Svaka opcija
se usklađuje prema svom `id`:

- Opcija poslana s `id` postojeće opcije zadržava tu opciju i njezine glasove. Njezina oznaka i pozicija
  ažuriraju se na ono što ste poslali.
- Opcija poslana bez `id` dodaje se, bez glasova.
- Postojeća opcija koju izostavite uklanja se, zajedno s glasovima danima na nju. `totalVotes` pada za isti
  iznos.

Dakle, za dodavanje opcije pošaljite trenutne opcije s njihovim id-jevima plus novu bez id-ja. Za uklanjanje
opcije pošaljite popis bez nje. Id-jevi opcija nalaze se na anketi vraćenoj putem `GET /api/v1/polls/:commentId`.

Ako uopće ne pošaljete id-jeve, svaka se opcija zamjenjuje i briše se svaki glas već dan na anketi. Ako anketa
ima glasove, to zahtijeva `replaceVotes=true`, a bez toga API odgovara s `replace-votes-required`.

Ostala polja također se zamjenjuju: izostavljanjem `closesAt`, `privacy` ili `requireVoteToSeeResults` resetira
se na zadanu vrijednost. Za promjenu jednog polja uz ostavljanje ostalih netaknutima, koristite `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Primjer Poll Put cURL zahtjeva'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura Poll Put zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura Poll Put odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Ostale napomene

- `id` koji nije na anketi, ili isti `id` dan dvaput, ne uspijeva s `poll-invalid`. Komentar bez
  ankete još nema id-jeve opcija, pa svaka opcija poslana njemu mora izostaviti `id`.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasove.
- Ovaj API poštuje postavke vaše stranice. Ako ankete nisu omogućene za stranicu ili stranicu, ne uspijeva s
  `polls-disabled`.
- Zaključanom komentaru ne može se promijeniti anketa i ne uspijeva s `locked`.
- Povezani widgeti ažuriraju se uživo, tako da gledatelji vide novu anketu bez ponovnog učitavanja.