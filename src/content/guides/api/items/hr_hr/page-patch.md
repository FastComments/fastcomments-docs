[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje ažuriranje jedne `Page`. Odgovarajući komentari bit će ažurirani.

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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    user?: Page; // Vraćamo kompletno ažuriranu stranicu pri uspjehu.
}
[inline-code-end]

#### Note

Neki parametri u objektu `Page` se automatski ažuriraju. To su brojači i atributi naslova. Brojače nije moguće ažurirati putem API-ja jer su to izračunate vrijednosti. `title` stranice može se postaviti putem API-ja, ali bit će prepisan ako se widget za komentare koristi na stranici s istim `urlId` i različitim naslovom stranice.