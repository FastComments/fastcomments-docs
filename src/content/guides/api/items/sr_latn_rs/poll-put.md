[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Prilaže anketu postojećem komentaru ili postavlja kompletno stanje ankete koju već ima.

Telo predstavlja kompletnu anketu, a opcije koje pošaljete postaju opcije ankete, redosledom kojim su navedene. Svaka opcija se podudara po svom `id`‑u:

- Opcija poslata sa `id`‑jem postojeće opcije zadržava tu opciju i njene glasove. Njena oznaka i pozicija se ažuriraju prema onome što ste poslali.
- Opcija poslata bez `id`‑ja se dodaje, bez glasova.
- Postojeća opcija koju izostavite se uklanja, zajedno sa glasovima koji su na nju dati. `totalVotes` opada za isti iznos.

Dakle, da biste dodali opciju, pošaljite trenutne opcije sa njihovim `id`‑jevima plus novu bez `id`‑ja. Da biste uklonili opciju, pošaljite listu bez nje. `id`‑je opcija se nalaze u anketi koju vraća `GET /api/v1/polls/:commentId`.

Slanje bez ikakvih `id`‑ja zamenjuje svaku opciju i briše sve glasove koji su već dati na anketu. Ako anketa ima glasove, ovo zahteva `replaceVotes=true`, a bez toga API odgovara sa `replace-votes-required`.

Ostala polja se takođe zamenjuju: izostavljanje `closesAt`, `privacy` ili `requireVoteToSeeResults` vraća njihovu podrazumevanu vrednost. Da biste izmenili samo jedno polje i ostala ostavili nepromenjena, koristite `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Primer cURL zahteva za Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Potrebno da se zadrže nijedni od postojećih id‑ova opcija kada anketa ima glasove, pošto to briše sve. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id postojeće opcije, da se zadrži ona i njeni glasovi. Izostavite da biste dodali novu opciju. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Kompletna, uređena lista. Izostavljene postojeće opcije se uklanjaju zajedno sa svojim glasovima. **/
    options: PollPutOption[]
    /** Mora biti u budućnosti kada komentar još nema anketu. Izostavite za anketu koja ostaje otvorena. **/
    closesAt?: string | null
    /** 0 anonimno (podrazumevano), 1 administratori i moderatori, 2 svi. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Uključeno u slučaju greške. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Ostale napomene

- `id` koji nije na anketi, ili isti `id` dat dva puta, izaziva grešku `poll-invalid`. Komentar bez ankete još nema `id`‑je opcija, pa svaka poslata opcija mora izostaviti `id`.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasove.
- Ovaj API poštuje podešavanja vašeg sajta. Ako ankete nisu omogućene za sajt ili stranicu, vraća grešku `polls-disabled`.
- Zaključani komentar ne može imati izmenjenu anketu i vraća grešku `locked`.
- Povezani vidžeti se ažuriraju u realnom vremenu, tako da posmatrači vide novu anketu bez ponovnog učitavanja.