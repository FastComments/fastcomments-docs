[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava ažuriranje jednog `Page`. Odgovarajući komentari će biti ažurirani.

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

[inline-code-attrs-start title = 'Struktura odgovora na ažuriranje stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Napomena

Neki parametri u Page objektu se automatski ažuriraju. To su atributi za brojače i naslov. Brojači se ne mogu ažurirati
putem API-ja jer su izračunate vrednosti. Naslov stranice (`title`) može se postaviti putem API-ja, ali biće prepisan ako se widget za komentare koristi
na stranici sa istim `urlId` ali drugačijim naslovom stranice.