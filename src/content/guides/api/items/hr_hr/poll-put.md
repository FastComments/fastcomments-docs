[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Prilaže anketu postojećem komentaru ili postavlja cijelo stanje ankete koju već ima.

Body je kompletna anketa, a opcije koje pošaljete postaju opcije ankete, u tom redoslijedu. Svaka opcija podudara se prema svom `id`-u:

- Opcija poslana s `id`-om postojeće opcije zadržava tu opciju i njene glasove. Njena oznaka i položaj ažuriraju se prema onome što ste poslali.
- Opcija poslana bez `id`-a se dodaje, bez glasova.
- Postojeća opcija koju izostavite uklanja se, zajedno s glasovima koji su na nju dati. `totalVotes` pada za isti iznos.

Dakle, da biste dodali opciju, pošaljite trenutne opcije s njihovim id-ovima plus novu bez id-a. Da biste uklonili opciju, pošaljite popis bez nje. Id-ovi opcija nalaze se u anketi koju vrati `GET /api/v1/polls/:commentId`.

Slanje bez ikakvih id-ova zamjenjuje svaku opciju i briše svaki glas koji je već dan na anketi. Ako anketa ima glasove, to zahtijeva `replaceVotes=true`, a bez toga API odgovara s `replace-votes-required`.

Ostala polja također se zamjenjuju: izostavljanje `closesAt`, `privacy` ili `requireVoteToSeeResults` vraća ih na zadane vrijednosti. Za promjenu jednog polja i ostavljanje ostalih nepromijenjenima, koristite `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za anketu (PUT)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za anketu (PUT)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za anketu (PUT)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- `id` koji nije u anketi, ili isti `id` naveden dvaput, uzrokuje grešku `poll-invalid`. Komentar bez ankete još nema id-ove opcija, pa svaka opcija poslana njemu mora izostaviti `id`.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasove.
- Ovaj API poštuje postavke vaše stranice. Ako ankete nisu omogućene za stranicu ili podstranicu, vraća grešku `polls-disabled`.
- Zaključani komentar ne može imati promijenjenu anketu i vraća grešku `locked`.
- Povezani widgeti se ažuriraju u stvarnom vremenu, pa posjetitelji vide novu anketu bez ponovnog učitavanja.