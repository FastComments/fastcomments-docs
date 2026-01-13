[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає до 100 об'єктів `QuestionConfig` за раз, з пагінацією. Вартість — 1 за кожні 100 об'єктів. Вони відсортовані за текстом питання у зростаючому порядку (поле `question`).

[inline-code-attrs-start title = 'Приклад QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Для пагінації. Починається з 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у випадку помилки. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]