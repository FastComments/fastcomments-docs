[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava ažuriranje jedne `Page`. Odgovarajući komentari će biti ažurirani.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za ažuriranje stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažuriranje stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Uključeno u slučaju greške. **/
    reason?: string
    user?: Page; // U slučaju uspjeha vraćamo kompletno ažuriranu stranicu.
}
[inline-code-end]

#### Napomena

Neki parametri u objektu Page se automatski ažuriraju. To su counts i title atributi. Counts cannot be updated
via the API since they are calculated values. The page `title` can be set via the API, but would get overwritten if the comment widget is used on
a page with the same `urlId` and a different page title.