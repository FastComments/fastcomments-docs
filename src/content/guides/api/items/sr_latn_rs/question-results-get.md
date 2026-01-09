[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Ovaj ruta vraća do 1000 `QuestionResults` objekata odjednom, paginirano. Cena je 1 za svaka 100 objekata. Oni su
sorted by `createdAt`, ascending. Možete filtrirati po raznim parametrima.

[inline-code-attrs-start title = 'Primer QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Za paginaciju. Počinje od 0. **/
    skip?: number
    /** Za paginaciju. **/
    limit?: number
    /** Dobijte rezultate sa određene stranice. **/
    urlId?: string
    /** Dobijte rezultate od određenog korisnika. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---