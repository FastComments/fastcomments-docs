[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ta končna točka API-ja omogoča ustvarjanje strani.

Pogosta uporaba je nadzor dostopa.

Opombe:

- Če ste komentirali v nit komentarjev, ali poklicali API za ustvarjanje `Comment`, ste že ustvarili objekt `Page`! Poskusite ga pridobiti prek `Page` poti `/by-url-id`, tako da posredujete isti `urlId`, ki je bil posredovan gradniku za komentarje.
- Struktura `Page` vsebuje nekaj **izračunanih** vrednosti.
  Trenutno so to `commentCount` in `rootCommentCount`.
  Te vrednosti se napolnijo samodejno in jih API ne dovoljuje nastaviti. Poskus nastavitev bo povzročil, da bo API vrnil napako.

[inline-code-attrs-start title = 'Primer cURL zahteve za Page POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura POST zahteve za Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Vključeno ob neuspehu. **/
    reason?: string
    /** Ustvarjena stran. **/
    page?: Page
}
[inline-code-end]