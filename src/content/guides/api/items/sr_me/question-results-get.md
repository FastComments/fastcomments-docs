[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Ова рута враћа до 1000 `QuestionResults` објеката одједном, у пагинираном облику. Цена је 1 на сваких 100 објеката. Они су сортирани по `createdAt`, у растућем редослиједу. Можете филтрирати по различитим параметрима.

[inline-code-attrs-start title = 'Пример QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** За пагинацију. Почиње од 0. **/
    skip?: number
    /** За пагинацију. **/
    limit?: number
    /** Добијте резултате са одређене странице. **/
    urlId?: string
    /** Добијте резултате од одређеног корисника. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---