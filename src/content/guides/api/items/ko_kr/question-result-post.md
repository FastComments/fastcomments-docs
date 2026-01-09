[api-resource-header-start name = 'QuestionResult'; route = 'POST /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 `QuestionResult`를 생성할 수 있는 기능을 제공합니다.

[inline-code-attrs-start title = 'QuestionResult POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "urlId": "some-page-id",
    "anonUserId": 'anon-0',
    "userId": 'user-0',
    "value": 10,
    "questionId": "some-question-id",
    "meta": [
        {
            name: "example",
            values: ["value-1", "value-2"]
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface QuestionResultPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input';  
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 생성된 객체. **/
    questionResult?: QuestionResult
}
[inline-code-end]