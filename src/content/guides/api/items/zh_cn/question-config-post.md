[api-resource-header-start name = 'QuestionConfig'; route = 'POST /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

此 API 端点提供创建 `QuestionConfig` 的能力。

[inline-code-attrs-start title = 'QuestionConfig POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionConfig POST 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig POST 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface QuestionConfigPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input';  
    /** 失败时包含。 **/
    reason?: string
    /** 已创建的对象。 **/
    questionConfig?: QuestionConfig
}
[inline-code-end]