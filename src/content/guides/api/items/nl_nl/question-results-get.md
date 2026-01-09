[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Deze route retourneert maximaal 1000 `QuestionResults` objecten tegelijk, gepagineerd. De kosten zijn 1 per 100 objecten. Ze worden
gesorteerd op `createdAt`, oplopend. Je kunt filteren op verschillende parameters.

[inline-code-attrs-start title = 'Voorbeeld van QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van QuestionResults-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Voor paginering. Begint bij 0. **/
    skip?: number
    /** Voor paginering. **/
    limit?: number
    /** Haal de resultaten op van een specifieke pagina. **/
    urlId?: string
    /** Haal de resultaten op van een specifieke gebruiker. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van QuestionResults-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij een fout. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---