[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

這裡是結果彙總的地方。

彙總回應結構如下：

[inline-code-attrs-start title = 'QuestionResultsAggregationResult 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 當前資料點（日期區段或頁面）中，將值映射到該值出現次數的映射。 **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Note - is null when timeBucket not specified. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 彙總的結果總數。 **/
    total: number
    /** 計算出的加權平均值。為浮點數，通常為兩位小數或更少。 **/
    average: number
    /** 表示此資料計算時間的日期字串，因為資料可能來自快取。 **/
    createdAt: string
}
[inline-code-end]

Here are the query parameters available for aggregation:

[inline-code-attrs-start title = 'QuestionResultsAggregation 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsAggregation 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Example response:

[inline-code-attrs-start title = 'QuestionResultsAggregation 回應範例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsAggregation 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### 效能注意事項

- 對於快取未命中，彙總通常每百萬筆結果約需五秒。
- 否則，請求為常數時間。

### 快取與費用說明

- 當 `forceRecalculate` 是指定時，費用恆為 `10`，而不是一般的 `2`。
- 如果快取過期且資料被重新計算，若未指定 `forceRecalculate`，費用仍為固定的 `2`。快取的過期時間取決於被彙總資料集的大小（可介於 30 秒到 5 分鐘之間）。
- 此機制是為了鼓勵使用快取。

---