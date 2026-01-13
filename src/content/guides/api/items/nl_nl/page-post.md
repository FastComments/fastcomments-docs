[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt maakt het mogelijk om pagina's aan te maken.

Een veelvoorkomend gebruiksgeval is toegangscontrole.

Opmerkingen:

- Als je op een comment-thread hebt gereageerd, of de API hebt aangeroepen om een `Comment` te maken, heb je al een `Page`-object aangemaakt! Je kunt proberen het op te halen via de `/by-url-id` `Page`-route, door hetzelfde `urlId` door te geven dat aan de commentaar-widget is doorgegeven.
- De `Page`-structuur bevat enkele **berekende** waarden.
  Momenteel zijn dit `commentCount` en `rootCommentCount`.
  Deze worden automatisch gevuld en kunnen niet door de API worden ingesteld. Pogingen daartoe zullen ertoe leiden dat de API een fout teruggeeft.

[inline-code-attrs-start title = 'cURL-voorbeeld voor Page POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Page POST-aanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Page POST-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** De aangemaakte pagina. **/
    page?: Page
}
[inline-code-end]