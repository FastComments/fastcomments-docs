[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Esta rota retorna até 1000 objetos `QuestionResults` por vez, paginados. O custo é 1 a cada 100 objetos. Eles são ordenados por `createdAt`, em ordem crescente. Você pode filtrar por vários parâmetros.

[inline-code-attrs-start title = 'Exemplo de QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Para paginação. Começa em 0. **/
    skip?: number
    /** Para paginação. **/
    limit?: number
    /** Obter os resultados de uma página específica. **/
    urlId?: string
    /** Obter os resultados de um usuário específico. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---