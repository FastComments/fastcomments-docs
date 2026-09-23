[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Redigerer en afstemning uden at forstyrre dens stemmer. Brug dette til at rette en stavefejl i spørgsmålet eller en mulighed, til at lukke eller
genåbne afstemningen, eller til at ændre, hvem der kan se, hvem der har stemt.

Muligheder adresseres via deres `id`, og en `PATCH` omdøber dem, du navngiver. For at tilføje, fjerne eller omarrangere
muligheder, send den fulde liste af muligheder til `PUT /api/v1/polls/:commentId`: muligheder du sender med deres id'er bevarer
deres stemmer også.

Alle felter er valgfrie, men mindst ét skal angives.

[inline-code-attrs-start title = 'Poll Patch cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Close A Poll Now cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Response Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Andre Bemærkninger

- At navngive et option-id, der ikke findes i afstemningen, fejler med `poll-invalid` i stedet for stille at gøre ingenting.
- Etiketter skal forblive unikke inden for afstemningen, også med de muligheder du ikke ændrer.
- I modsætning til at oprette en afstemning, kan `closesAt` her være i fortiden – det er sådan du lukker en afstemning med det samme.
- Afstemningsprivatliv kan indsnævres men ikke udvides, når den har stemmer.
- En låst kommentar kan ikke få sin afstemning ændret, og fejler med `locked`.

---