[api-resource-header-start name = 'QuestionConfig'; route = 'PATCH /api/v1/question-configs/:id'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 `QuestionConfig`를 업데이트하는 기능을 제공합니다.

다음 구조는 변경 가능한 모든 값을 나타냅니다:

[inline-code-attrs-start title = 'QuestionConfig 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchBody {
    name?: string
    question?: string
    helpText?: string
    /** 주의: type을 변경하면 min/max가 다를 경우 보고에 영향을 줄 수 있습니다. **/
    type?: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 업데이트 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-configs/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "some new question text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 업데이트 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig 업데이트 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---