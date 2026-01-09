[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint vraća do 100 `QuestionConfig` objekata odjednom, paginirano. Trošak je 1 za svaka 100 objekata. Razvrstani su po tekstu pitanja uzlazno (`question` field).

[inline-code-attrs-start title = 'Primjer QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Za paginaciju. Počinje od 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---