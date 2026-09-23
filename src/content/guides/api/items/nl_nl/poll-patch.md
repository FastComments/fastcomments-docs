[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Bewerk een poll zonder de stemmen te verstoren. Gebruik dit om een typefout in de vraag of een optie te corrigeren, om de poll te sluiten of te heropenen, of om te wijzigen wie mag zien wie heeft gestemd.

Opties worden aangesproken via hun `id`, en een `PATCH` hernoemt de door jou genoemde. Om opties toe te voegen, te verwijderen of te herschikken, stuur je de volledige optielijst naar `PUT /api/v1/polls/:commentId`: opties die je met hun id's stuurt behouden hun stemmen ook.

Elk veld is optioneel, maar er moet minstens één worden opgegeven.

[inline-code-attrs-start title = 'Poll Patch cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll nu sluiten cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Poll Patch Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Other Notes

- Het benoemen van een optie-id die niet in de poll aanwezig is, resulteert in een fout `poll-invalid` in plaats van stilletjes niets te doen.
- Labels moeten uniek blijven binnen de poll, inclusief de opties die je niet wijzigt.
- In tegenstelling tot het aanmaken van een poll, mag `closesAt` hier in het verleden liggen – dat is hoe je een poll onmiddellijk sluit.
- Poll-privacy kan worden vernauwd maar niet verbreed zodra er stemmen zijn.
- Een vergrendelde reactie kan niet van poll worden veranderd, en resulteert in een fout `locked`.