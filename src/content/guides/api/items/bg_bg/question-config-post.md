[api-resource-header-start name = 'QuestionConfig'; route = 'POST /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за създаване на `QuestionConfig`.

[inline-code-attrs-start title = 'Пример за POST на QuestionConfig с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some name that shows on reports.",
    "question": "how much do you like this api?",
    "type": 'slider',
    "reportingOrder": 0,
    "min": 0,
    "max": 10
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за POST на QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за POST на QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface QuestionConfigPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input';
    /** Included on failure. **/
    reason?: string
    /** The created object. **/
    questionConfig?: QuestionConfig
}
[inline-code-end]
