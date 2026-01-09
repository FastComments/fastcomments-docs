[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Questa route restituisce fino a 1000 oggetti `QuestionResults` alla volta, paginati. Il costo è 1 ogni 100 oggetti. Sono
ordinati per `createdAt`, in ordine crescente. Puoi filtrare per vari parametri.

[inline-code-attrs-start title = 'Esempio di QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Per la paginazione. Inizia da 0. **/
    skip?: number
    /** Per la paginazione. **/
    limit?: number
    /** Ottieni i risultati da una pagina specifica. **/
    urlId?: string
    /** Ottieni i risultati da un utente specifico. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---