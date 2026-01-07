[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Този маршрут връща до 100 обекта `QuestionConfig` наведнъж, пагинирани. Цената е 1 за всеки 100 обекта. Те са
сортирани по текста на въпроса във възходящ ред (поле `question`).

[inline-code-attrs-start title = 'Пример за QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** For pagination. Starts at 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]
