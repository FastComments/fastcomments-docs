[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Bewerkt een poll zonder de stemmen te verstoren. Gebruik dit om een typfout in de vraag of een optie te herstellen, om de poll te sluiten of
heropenen, of om te wijzigen wie mag zien wie er heeft gestemd.

Opties worden aangeduid met hun `id`, en een `PATCH` hernoemt degene die je opgeeft. Om opties toe te voegen, te verwijderen of te
herordenen, stuur je de volledige optielijst naar `PUT /api/v1/polls/:commentId`: opties die je met hun ids stuurt behouden
hun stemmen daar ook.

Elk veld is optioneel, maar er moet er minstens één worden opgegeven.

[inline-code-attrs-start title = 'Poll Patch cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Nu Sluiten cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Hernoemt bestaande opties. Elke opgegeven id moet al op de poll bestaan. **/
    options?: { id: string, label: string }[] | null
    /** Een datum in het verleden sluit de poll nu. null heropent een gesloten poll. **/
    closesAt?: string | null
    /** 0 anoniem, 1 admins en moderatoren, 2 iedereen. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Inbegrepen bij fout. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Overige Opmerkingen

- Het opgeven van een optie-id die niet op de poll staat, mislukt met `poll-invalid` in plaats van stilletjes niets te doen.
- Labels moeten uniek blijven binnen de poll, inclusief de opties die je niet wijzigt.
- In tegenstelling tot bij het aanmaken van een poll mag `closesAt` hier in het verleden liggen - zo sluit je een poll onmiddellijk.
- De privacy van een poll kan worden versmald maar niet verbreed zodra er stemmen zijn.
- Een vergrendelde reactie kan niet worden gewijzigd en mislukt met `locked`.