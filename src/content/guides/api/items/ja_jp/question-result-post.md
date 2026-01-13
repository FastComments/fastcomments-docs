---
[api-resource-header-start name = 'QuestionResult'; route = 'POST /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは、`QuestionResult` を作成する機能を提供します。

[inline-code-attrs-start title = 'QuestionResult の POST cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResult POST リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult POST レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface QuestionResultPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input';  
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 作成されたオブジェクト。 **/
    questionResult?: QuestionResult
}
[inline-code-end]

---