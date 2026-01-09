---
[api-resource-header-start name = 'QuestionResult'; route = 'DELETE /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Esta rota realiza a remoção de um `QuestionResult` pelo id.

[inline-code-attrs-start title = 'Exemplo cURL de Remoção de QuestionResult'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-results/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Remoção de QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Remoção de QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---