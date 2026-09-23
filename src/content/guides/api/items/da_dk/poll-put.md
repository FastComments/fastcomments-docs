[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Vedhæfter en afstemning til en eksisterende kommentar, eller indstiller den fulde tilstand af den afstemning, den allerede har.

Kroppen er den komplette afstemning, og de muligheder du sender bliver afstemningens muligheder i den rækkefølge. Hver mulighed matches med dens `id`:

- En mulighed sendt med `id` for en eksisterende mulighed bevarer den mulighed og dens stemmer. Dens etiket og position opdateres til det, du sendte.
- En mulighed sendt uden et `id` tilføjes, uden stemmer.
- En eksisterende mulighed du udelader fjernes, sammen med de stemmer der er afgivet på den. `totalVotes` falder med samme beløb.

Så for at tilføje en mulighed, send de aktuelle muligheder med deres id'er plus den nye uden et id. For at fjerne en, send listen uden den. Muligheds-id'erne findes på afstemningen returneret af `GET /api/v1/polls/:commentId`.

At sende ingen id'er overhovedet erstatter hver mulighed og sletter hver stemme, der allerede er afgivet på afstemningen. Hvis afstemningen har stemmer, kræver dette `replaceVotes=true`, og uden det svarer API'en med `replace-votes-required`.

De andre felter erstattes også: at udelade `closesAt`, `privacy` eller `requireVoteToSeeResults` nulstiller dem til standardværdien. For at ændre et enkelt felt og lade resten være uændret, brug `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Poll Put cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Påkrævet for at beholde ingen af de eksisterende muligheds-id'er, når afstemningen har stemmer, da det sletter dem alle. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id'et for en eksisterende mulighed, for at beholde den og dens stemmer. Udelad for at tilføje en ny mulighed. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Den komplette, ordnede liste. Eksisterende muligheder, der udelades, fjernes sammen med deres stemmer. **/
    options: PollPutOption[]
    /** Skal være i fremtiden, når kommentaren endnu ikke har en afstemning. Udelad for en afstemning, der forbliver åben. **/
    closesAt?: string | null
    /** 0 anonym (standard), 1 administratorer og moderatorer, 2 alle. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put Responsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Inkluderet ved fejl. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Et `id`, der ikke findes på afstemningen, eller det samme `id` givet to gange, fejler med `poll-invalid`. En kommentar uden afstemning har endnu ingen muligheds-id'er, så hver mulighed, der sendes til den, skal udelade `id`.
- Afstemningsprivatliv kan indsnævres men ikke udvides, når den har stemmer.
- Dette API overholder dine sideindstillinger. Hvis afstemninger ikke er aktiveret for siden eller siden, fejler det med `polls-disabled`.
- En låst kommentar kan ikke få sin afstemning ændret, og fejler med `locked`.
- Forbundne widgets opdateres live, så seere ser den nye afstemning uden at genindlæse.