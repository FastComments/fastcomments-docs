[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia aktualizację pojedynczego `Page`. Odpowiadające komentarze zostaną zaktualizowane.

[inline-code-attrs-start title = 'Przykład cURL aktualizacji strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania aktualizacji strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi aktualizacji strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Uwaga

Niektóre parametry w obiekcie Page są automatycznie aktualizowane. Są to atrybuty counts i title. Wartości counts nie mogą być aktualizowane
przez API, ponieważ są to wartości obliczane. Pole `title` strony można ustawić przez API, ale zostanie nadpisane, jeśli widżet komentarzy zostanie użyty
na stronie z tym samym `urlId` i innym tytułem strony.