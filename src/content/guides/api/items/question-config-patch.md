[api-resource-header-start name = 'QuestionConfig'; route = 'PATCH /api/v1/question-configs/:id'; creditsCost = 1; api-resource-header-end]

This route provides the ability to update a single `QuestionConfig`.

The following structure represents all values that can be changed:

[inline-code-attrs-start title = 'QuestionConfig Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchBody {
    name?: string
    question?: string
    helpText?: string
    /** Beware! Changing type may throw off reporting if min/max is different. **/
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

[inline-code-attrs-start title = 'QuestionConfig Update cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-configs/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "some new question text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Update Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Update Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
