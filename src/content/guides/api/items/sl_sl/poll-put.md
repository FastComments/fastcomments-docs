[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Pripne anketo k obstoječemu komentarju ali nastavi celotno stanje ankete, ki jo že ima.

Telo je celotna anketa, možnosti, ki jih pošljete, postanejo možnosti ankete v tem vrstnem redu. Vsaka možnost
je ujemana po njenem `id`:

- Možnost, poslana z `id` obstoječe možnosti, ohrani to možnost in njene glasove. Njena oznaka in položaj
  sta posodobljena na podlagi poslanega.
- Možnost, poslana brez `id`, se doda, brez glasov.
- Obstoječo možnost, ki jo izpustite, se odstrani skupaj z glasovi, ki so bili oddani zanjo. `totalVotes` se zmanjša za enako
  količino.

Torej, da dodate možnost, pošljite trenutne možnosti z njihovimi ID-ji plus novo brez ID-ja. Da odstranite eno,
pošljite seznam brez nje. ID-ji možnosti so v anketi, vrnjeni z `GET /api/v1/polls/:commentId`.

Pošiljanje brez ID-jev povsem nadomesti vsako možnost in izbriše vsak glas, ki je bil že oddan v anketi. Če ima anketa
glasove, je to potrebno `replaceVotes=true`, brez tega API odgovori z `replace-votes-required`.

Ostala polja se prav tako nadomestijo: izpuščanje `closesAt`, `privacy` ali `requireVoteToSeeResults` ga ponastavi na
privzeto vrednost. Za spremembo enega polja in ohranitev ostalih, uporabite `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Primer cURL zahteve za anketo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteve za anketo (PUT)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Zahtevano, da se ne ohranijo nobeni obstoječi ID-ji možnosti, ko anketa ima glasove, saj bi to izbrisalo vse. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** ID obstoječe možnosti, da jo ohranimo skupaj z njenimi glasovi. Izpustite, da dodate novo možnost. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Celoten, urejen seznam. Izpuščene obstoječe možnosti se odstranijo skupaj z njihovimi glasovi. **/
    options: PollPutOption[]
    /** Mora biti v prihodnosti, ko komentar še nima ankete. Izpustite za anketo, ki ostane odprta. **/
    closesAt?: string | null
    /** 0 anonimno (privzeto), 1 administratorji in moderatorji, 2 vsi. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za anketo (PUT)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Vključeno ob napaki. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Ostale opombe

- `id`, ki ni v anketi, ali podvojen `id`, povzroči napako `poll-invalid`. Komentar brez
  ankete še nima ID-jev možnosti, zato mora vsaka poslana možnost izpustiti `id`.
- Zasebnost ankete je mogoče zožiti, vendar ne razširiti, ko ima glasove.
- Ta API spoštuje nastavitve vašega spletnega mesta. Če ankete niso omogočene za spletno mesto ali stran, vrne napako
  `polls-disabled`.
- Zaklenjen komentar ne more imeti spremenjene ankete in vrne napako `locked`.
- Povezani pripomočki se posodabljajo v živo, tako da si gledalci ogledajo novo anketo brez ponovnega nalaganja.