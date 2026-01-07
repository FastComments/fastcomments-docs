[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at oprette sider.

Et almindeligt anvendelsestilfælde er adgangskontrol.

Bemærkninger:

- Hvis du har kommenteret på en kommentartråd eller kaldt API'et for at oprette en `Comment`, har du allerede oprettet et `Page`-objekt! Du kan prøve at hente det via
  `/by-url-id` `Page`-ruten ved at angive det samme `urlId`, som blev sendt til kommentar-widget'en.
- `Page`-strukturen indeholder nogle **beregnede** værdier.
  I øjeblikket er disse `commentCount` og `rootCommentCount`.
  De udfyldes automatisk og kan ikke sættes af API'et. Forsøg på at gøre det vil få API'et til at returnere en fejl.

[inline-code-attrs-start title = 'Page POST cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Page POST Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
