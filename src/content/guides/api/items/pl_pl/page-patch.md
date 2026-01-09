[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa zapewnia możliwość aktualizacji pojedynczej `Page`. Odpowiadające jej komentarze zostaną zaktualizowane.

[inline-code-attrs-start title = 'Przykład żądania cURL aktualizacji strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    user?: Page; // Zwracamy kompletnie zaktualizowaną stronę w przypadku powodzenia.
}
[inline-code-end]

#### Note

Some parameters in the Page object get automatically updated. These are the counts and title attributes. Counts cannot be updated
via the API since they are calculated values. The page `title` can be set via the API, but would get overwritten if the comment widget is used on
a page with the same `urlId` and a different page title.