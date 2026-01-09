[api-resource-header-start name = 'QuestionResult'; route = 'GET /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Questa route restituisce un singolo `QuestionResult` in base al suo id.

[inline-code-attrs-start title = 'Esempio cURL di QuestionResult per ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultByIdResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    questionResult?: QuestionResult
}
[inline-code-end]

---