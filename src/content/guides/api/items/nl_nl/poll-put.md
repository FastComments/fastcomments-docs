[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Voegt een poll toe aan een bestaande reactie, of stelt de volledige status van de poll in die het al heeft.

De body is de volledige poll, en de opties die je verzendt worden de opties van de poll, in die volgorde. Elke optie wordt gematcht op zijn `id`:

- Een optie die wordt verzonden met de `id` van een bestaande optie behoudt die optie en zijn stemmen. Het label en de positie worden bijgewerkt naar wat je hebt verzonden.
- Een optie die zonder een `id` wordt verzonden, wordt toegevoegd, zonder stemmen.
- Een bestaande optie die je weglaat, wordt verwijderd, samen met de daarop uitgebrachte stemmen. `totalVotes` daalt met hetzelfde aantal.

Dus om een optie toe te voegen, stuur je de huidige opties met hun ids plus de nieuwe zonder een id. Om er een te verwijderen, stuur je de lijst zonder die optie. De optie‑ids staan in de poll die wordt geretourneerd door `GET /api/v1/polls/:commentId`.

Het verzenden van helemaal geen ids vervangt elke optie en verwijdert elke stem die al op de poll is uitgebracht. Als de poll stemmen heeft, vereist dit `replaceVotes=true`, en zonder dit reageert de API met `replace-votes-required`.

De andere velden worden ook vervangen: het weglaten van `closesAt`, `privacy` of `requireVoteToSeeResults` zet deze terug naar de standaardwaarde. Om één veld te wijzigen en de rest ongewijzigd te laten, gebruik je `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Overige opmerkingen

- Een `id` die niet in de poll voorkomt, of dezelfde `id` die twee keer wordt opgegeven, faalt met `poll-invalid`. Een reactie zonder poll heeft nog geen optie‑ids, dus elke optie die ernaar wordt verzonden moet `id` weglaten.
- Poll‑privacy kan worden verscherpt maar niet verbreed zodra er stemmen zijn.
- Deze API volgt de instellingen van je site. Als polls niet zijn ingeschakeld voor de site of pagina, faalt het met `polls-disabled`.
- Een vergrendelde reactie kan zijn poll niet wijzigen, en faalt met `locked`.
- Verbonden widgets worden live bijgewerkt, zodat kijkers de nieuwe poll zien zonder te herladen.

---