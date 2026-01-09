---
[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 100 `QuestionConfig` объектов за раз, с пагинацией. Стоимость — 1 за каждые 100 объектов. Они отсортированы по тексту вопроса по возрастанию (`question` field).

[inline-code-attrs-start title = 'Пример QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** Для пагинации. Начинается с 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Указывается при ошибке. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]

---