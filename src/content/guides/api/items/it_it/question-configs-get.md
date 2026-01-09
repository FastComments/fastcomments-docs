[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Questa route restituisce fino a 100 oggetti `QuestionConfig` alla volta, paginati. Il costo è 1 per ogni 100 oggetti. Sono
ordinati per testo della domanda in ordine ascendente (`question` field).

[inline-code-attrs-start title = 'Esempio di QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Per la paginazione. Parte da 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]