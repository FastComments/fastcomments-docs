[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає до 1000 об'єктів `QuestionResults` за раз, з пагінацією. Вартість — 1 за кожні 100 об'єктів. Вони
відсортовані за `createdAt`, у зростаючому порядку. Ви можете фільтрувати за різними параметрами.

[inline-code-attrs-start title = 'Приклад QuestionResults'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Для пагінації. Починається з 0. **/
    skip?: number
    /** Для пагінації. **/
    limit?: number
    /** Отримати результати конкретної сторінки. **/
    urlId?: string
    /** Отримати результати конкретного користувача. **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді QuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** Включається при помилці. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається при помилці. **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]

---