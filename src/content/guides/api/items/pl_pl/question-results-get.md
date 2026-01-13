[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Ta trasa zwraca maksymalnie 1000 obiektów `QuestionResults` jednocześnie, stronicowanych. Koszt to 1 za każde 100 obiektów. Są
posortowane według `createdAt`, rosnąco. Możesz filtrować według różnych parametrów.

[inline-code-attrs-start title = 'Przykład QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Do paginacji. Zaczyna się od 0. **/
    skip?: number
    /** Do paginacji. **/
    limit?: number
    /** Pobierz wyniki z konkretnej strony. **/
    urlId?: string
    /** Pobierz wyniki od konkretnego użytkownika. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---