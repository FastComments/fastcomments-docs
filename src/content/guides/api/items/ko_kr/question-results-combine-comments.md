[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

결과와 댓글을 결합하는 기능이 제공되는 엔드포인트입니다. 예를 들어 제품에 대한 "최근 긍정 및 부정 댓글" 차트를 만드는 데 유용합니다.

값의 범위(포함), 하나 이상의 질문, 그리고 시작 날짜(포함)로 검색할 수 있습니다.

응답 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'QuestionResultsCombinedWithCommentsResponse 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** 이 데이터가 계산된 시점을 나타내는 날짜 문자열입니다(캐시에서 제공될 수 있음). **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

다음은 집계를 위한 쿼리 매개변수입니다:

[inline-code-attrs-start title = 'QuestionResultsCombineComments 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 하나 이상의 질문에 대해 결과를 집계할 수 있습니다. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

다음은 예시 요청입니다:

[inline-code-attrs-start title = 'QuestionResultsCombineComments 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

다음은 예시 응답입니다:

[inline-code-attrs-start title = 'QuestionResultsCombineComments 응답 예제'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsCombineComments 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### 캐싱 및 비용 관련 주의사항

- `forceRecalculate`가 지정된 경우 비용은 일반적인 `2` 대신 항상 `10`입니다.
- 캐시가 만료되어 데이터가 재계산되는 경우에도, `forceRecalculate`가 지정되지 않았다면 비용은 여전히 고정된 `2`입니다.
- 이는 캐시 사용을 장려하기 위한 조치입니다.

---