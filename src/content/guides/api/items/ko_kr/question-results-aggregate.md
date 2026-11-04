[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

여기서 결과 집계가 이루어집니다.

집계 응답 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 현재 데이터 포인트(날짜 버킷 또는 페이지)에서 해당 값의 발생 횟수를 값별로 매핑한 것입니다. **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** 참고 - timeBucket이 지정되지 않은 경우 null입니다. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 집계된 결과의 총 개수입니다. **/
    total: number
    /** 계산된 가중 평균입니다. 부동 소수점 수이며, 보통 소수점 둘째 자리 이하입니다. **/
    average: number
    /** 이 데이터가 계산된 시점을 나타내는 날짜 문자열입니다. 캐시에서 가져올 수 있기 때문입니다. **/
    createdAt: string
}
[inline-code-end]

집계에 사용할 수 있는 쿼리 파라미터는 다음과 같습니다:

[inline-code-attrs-start title = 'QuestionResultsAggregation 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 하나 이상의 질문에 대해 결과를 집계할 수 있습니다. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** 특정 페이지에 대해 집계합니다. **/
    urlId?: string
    /** 특정 사용자에 대해 집계합니다. **/
    userId?: string
    /** 지금 즉시 재계산하고 캐시를 업데이트합니다. **/
    forceRecalculate?: boolean
}
[inline-code-end]

요청 예시는 다음과 같습니다:

[inline-code-attrs-start title = 'QuestionResultsAggregation 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

응답 예시:

[inline-code-attrs-start title = 'QuestionResultsAggregation 응답 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    {
        "average": 8.33,
        "countsByValue": {
            "5": 1,
            "10": 2
        },
        "createdAt": "2023-08-30T00:00:00.000Z",
        "dataByUrlId": {
            "some-page": {
                "total": 3,
                "v": {
                    "5": 1,
                    "10": 2
                }
            }
        },
        "total": 3
    }
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsAggregation 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### 성능 참고 사항

- 캐시 미스 시 집계는 일반적으로 백만 개의 결과당 약 5초가 걸립니다.
- 그 외의 경우 요청은 일정한 시간 내에 처리됩니다.

### 캐싱 및 비용 참고 사항

- `forceRecalculate`가 지정되면 비용은 일반적인 `2` 대신 항상 `10`입니다.
- 캐시가 만료되어 데이터가 재계산되더라도, `forceRecalculate`가 지정되지 않은 경우 비용은 여전히 일정하게 `2`입니다. 캐시는 집계된 데이터 세트 크기에 따라 만료됩니다(30초에서 5분 사이로 변동 가능).
- 이는 캐시 사용을 장려하기 위한 것입니다.