[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia tworzenie stron.

Typowym przypadkiem użycia jest kontrola dostępu.

Uwagi:

- Jeśli skomentowałeś wątek komentarzy lub wywołałeś API w celu utworzenia `Comment`, to już utworzyłeś obiekt `Page`! Możesz spróbować pobrać go za pomocą
  trasy `Page` `/by-url-id`, przekazując ten sam `urlId`, który został przekazany do widżetu komentarzy.
- Struktura `Page` zawiera niektóre wartości **obliczane**.
  Obecnie są to `commentCount` i `rootCommentCount`.
  Są one wypełniane automatycznie i nie można ich ustawić przez API. Próba zrobienia tego spowoduje zwrócenie błędu przez API.

[inline-code-attrs-start title = 'Przykład cURL POST dla Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    /** Utworzona strona. **/
    page?: Page
}
[inline-code-end]