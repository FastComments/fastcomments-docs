[api-resource-header-start name = 'QuestionConfig'; route = 'DELETE /api/v1/question-configs/:id'; creditsCost = 100; api-resource-header-end]

Этот маршрут позволяет удалить `QuestionConfig` по id.

**Это удалит все соответствующие результаты вопросов (но не комментарии).** Это связано с высокой стоимостью в кредитах.

[inline-code-attrs-start title = 'Пример cURL запроса удаления QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-configs/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса удаления QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при удалении QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]

---