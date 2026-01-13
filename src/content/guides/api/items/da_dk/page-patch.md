[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at opdatere en enkelt `Page`. De tilsvarende kommentarer vil blive opdateret.

[inline-code-attrs-start title = 'Page Opdatering cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page Opdatering Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Opdatering Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Included on failure. **/
    reason?: string
    user?: Page; // We return the complete updated page on success.
}
[inline-code-end]

#### Bemærkning

Nogle parametre i Page-objektet opdateres automatisk. Disse er tælle- og titelattributter. Tællere kan ikke opdateres
via API'et, da de er beregnede værdier. Sidens `title` kan sættes via API'et, men vil blive overskrevet, hvis kommentar-widget'en bruges på
en side med det samme `urlId` og en anden sidetitel.
