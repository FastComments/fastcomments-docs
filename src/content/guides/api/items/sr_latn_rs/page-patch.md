[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava ažuriranje jedne `Page`. Odgovarajući komentari će biti ažurirani.

[inline-code-attrs-start title = 'Primer cURL zahteva za ažuriranje stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za ažuriranje stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    user?: Page; // Vraćamo kompletan ažurirani Page pri uspehu.
}
[inline-code-end]

#### Napomena

Neki parametri u Page objektu se automatski ažuriraju. To su atributi counts i title. Counts se ne mogu ažurirati
putem API-ja jer su to izračunate vrednosti. The page `title` can be set via the API, but would get overwritten if the comment widget is used on
a page with the same `urlId` and a different page title.