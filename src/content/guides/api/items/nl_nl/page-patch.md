[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele `Page` bij te werken. De bijbehorende reacties worden bijgewerkt.

[inline-code-attrs-start title = 'cURL-voorbeeld voor Pagina-update'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina-updateverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina-updateantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Opgenomen bij mislukking. **/
    reason?: string
    user?: Page; // We geven de volledig bijgewerkte pagina terug bij succes.
}
[inline-code-end]

#### Opmerking

Sommige parameters in het Page-object worden automatisch bijgewerkt. Dit zijn de aantallen en de title-attributen. Aantallen kunnen niet via de API worden bijgewerkt omdat het berekende waarden zijn. De pagina `title` kan via de API worden ingesteld, maar wordt overschreven als de reactiewidget wordt gebruikt op een pagina met dezelfde `urlId` en een andere paginatitel.