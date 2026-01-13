[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Obecnie możesz pobrać tylko wszystkie strony (lub pojedynczą stronę przez `/by-url-id`) powiązane z Twoim kontem. Jeśli chciałbyś bardziej precyzyjnych możliwości wyszukiwania, [skontaktuj się z nami](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Przykład żądania cURL dla stron'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Przydatna wskazówka

API `Comment` wymaga `urlId`. Możesz najpierw wywołać API `Pages`, aby zobaczyć, jak wyglądają dostępne dla Ciebie wartości `urlId`.