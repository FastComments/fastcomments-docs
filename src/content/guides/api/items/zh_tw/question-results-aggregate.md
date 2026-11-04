[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

這裡是結果聚合的地方。

聚合回應的結構如下：

[inline-code-attrs-start title = 'QuestionResultsAggregationResult 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 目前資料點(日期區間或頁面)中,每個值出現次數的映射。 **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** 注意 - 未指定 timeBucket 時為 null。 **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 聚合的結果總數。 **/
    total: number
    /** 加權平均結果。為浮點數,通常不超過兩位小數。 **/
    average: number
    /** 代表此資料計算時間的日期字串,因為可能來自快取。 **/
    createdAt: string
}
[inline-code-end]

以下是聚合可用的查詢參數:

[inline-code-attrs-start title = 'QuestionResultsAggregation 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以針對一或多個問題聚合結果。 **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** 針對特定頁面進行聚合。 **/
    urlId?: string
    /** 針對特定使用者進行聚合。 **/
    userId?: string
    /** 強制立即重新計算並更新快取。 **/
    forceRecalculate?: boolean
}
[inline-code-end]

以下是請求範例:

[inline-code-attrs-start title = 'QuestionResultsAggregation 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

回應範例:

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
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時包含。 **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### 效能說明

- 快取未命中時,每百萬筆結果的聚合通常需要五秒鐘。
- 否則,請求為常數時間。

### 快取與成本說明

- 當指定 `forceRecalculate` 時,成本一律為 `10`,而非一般的 `2`。
- 如果快取過期且資料被重新計算,在未指定 `forceRecalculate` 的情況下,成本仍為固定的 `2`。快取會根據聚合的資料集大小過期(介於 30 秒到 5 分鐘之間)。
- 這是為了鼓勵使用快取。