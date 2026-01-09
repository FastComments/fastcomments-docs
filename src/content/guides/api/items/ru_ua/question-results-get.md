[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 1000 объектов `QuestionResults` за раз, с пагинацией. Стоимость — 1 за каждые 100 объектов. Они отсортированы по `createdAt` в порядке возрастания. Вы можете фильтровать по различным параметрам.

[inline-code-attrs-start title = 'Пример QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Для пагинации. Нумерация начинается с 0. **/
    skip?: number
    /** Для пагинации. **/
    limit?: number
    /** Получить результаты с конкретной страницы. **/
    urlId?: string
    /** Получить результаты от конкретного пользователя. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Присутствует при ошибке. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]