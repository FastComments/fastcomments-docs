[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Vedhæfter en afstemning til en eksisterende kommentar, eller sætter den fulde tilstand for den afstemning, den allerede har.

Body'en er den komplette afstemning, og de indstillinger, du sender, bliver afstemningens indstillinger, i den rækkefølge. Hver indstilling
matches via dens `id`:

- En indstilling sendt med `id` for en eksisterende indstilling bevarer den indstilling og dens stemmer. Dens label og position
  opdateres til det, du sendte.
- En indstilling sendt uden et `id` tilføjes uden stemmer.
- En eksisterende indstilling, du udelader, fjernes sammen med de afgivne stemmer på den. `totalVotes` falder med det samme
  beløb.

Så for at tilføje en indstilling, send de nuværende indstillinger med deres ids plus den nye uden et id. For at fjerne en,
send listen uden den. Indstillings-id'erne findes på afstemningen returneret af `GET /api/v1/polls/:commentId`.

Hvis du slet ikke sender nogen id'er, erstattes alle indstillinger, og alle allerede afgivne stemmer på afstemningen slettes. Hvis afstemningen har
stemmer, kræver dette `replaceVotes=true`, og uden det svarer API'en med `replace-votes-required`.

De andre felter erstattes også: udeladelse af `closesAt`, `privacy` eller `requireVoteToSeeResults` nulstiller det til
dets standardværdi. For at ændre et enkelt felt og lade resten være, brug `PATCH /api/v1/polls/:commentId`.

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

[inline-code-attrs-start title = 'Poll Put Request Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Put Svar Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Andre Bemærkninger

- Et `id`, der ikke findes på afstemningen, eller det samme `id` givet to gange, fejler med `poll-invalid`. En kommentar uden
  afstemning har endnu ingen indstillings-id'er, så alle indstillinger sendt til den skal udelade `id`.
- Afstemningens privatliv kan indsnævres, men ikke udvides, når den har stemmer.
- Denne API følger dine webstedsindstillinger. Hvis afstemninger ikke er aktiveret for webstedet eller siden, fejler den med
  `polls-disabled`.
- En låst kommentar kan ikke få sin afstemning ændret og fejler med `locked`.
- Tilsluttede widgets opdateres live, så seere ser den nye afstemning uden at genindlæse.