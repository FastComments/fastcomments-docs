[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Овај рутa враћа до 1000 `QuestionResults` објеката одједном, пагинирано. Трошак је 1 за свакaких 100 објеката. Сортирано по `createdAt`, узлазно. Можете филтрирати по разним параметрима.

[inline-code-attrs-start title = 'QuestionResults Пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults Структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** За пагинацију. Почиње од 0. **/
    skip?: number
    /** За пагинацију. **/
    limit?: number
    /** Добијте резултате за одређену страницу. **/
    urlId?: string
    /** Добијте резултате од одређеног корисника. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults Структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспеха. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]