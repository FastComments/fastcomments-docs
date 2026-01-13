[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

여기서 결과의 집계가 수행됩니다.

The aggregation response structure is as follows:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 현재 데이터 포인트(날짜 버킷 또는 페이지)에 대해 값별 발생 횟수를 저장한 맵. **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** 참고 - timeBucket이 지정되지 않으면 null입니다. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 집계된 결과의 총 수입니다. **/
    total: number
    /** 계산된 가중 평균입니다. 보통 소수점 둘째 자리 또는 그 이하입니다. **/
    average: number
    /** 데이터가 캐시에서 제공될 수 있으므로 이 데이터가 계산된 시점을 나타내는 날짜 문자열입니다. **/
    createdAt: string
}
[inline-code-end]

Here are the query parameters available for aggregation:

[inline-code-attrs-start title = 'QuestionResultsAggregation 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Aggregate for a specific page. **/
    urlId?: string
    /** Aggregate for a specific user. **/
    userId?: string
    /** Force recalculate now and update the cache. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Here's an example request:

[inline-code-attrs-start title = 'QuestionResultsAggregation 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Example response:

[inline-code-attrs-start title = 'QuestionResultsAggregation 응답 예제'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Performance Notes

- For a cache miss aggregations generally take five seconds per million results.
- Otherwise, requests are constant-time.

### Caching and Cost Notes

- When `forceRecalculate` is specified the cost is always `10`, instead of the normal `2`.
- If the cache expires and data is recalculated, the cost is still a constant `2` if `forceRecalculate` is not specified. The cache expires based on the data set size aggregated (can vary between 30 seconds and 5 minutes).
- This is to incentivize using the cache.

---