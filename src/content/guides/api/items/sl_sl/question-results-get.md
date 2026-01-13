[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Ta ruta vrne do 1000 `QuestionResults` objektov naenkrat, paginirano. Strošek je 1 za vsakih 100 objektov. Razvrščeni so po `createdAt`, naraščajoče. Lahko filtrirate po različnih parametrih.

[inline-code-attrs-start title = 'Primer QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Za straničenje. Začne se pri 0. **/
    skip?: number
    /** Za straničenje. **/
    limit?: number
    /** Pridobi rezultate s določene strani. **/
    urlId?: string
    /** Pridobi rezultate določenega uporabnika. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Vključen ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključen ob napaki. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]