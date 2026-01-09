[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

这里进行结果的聚合。

聚合响应结构如下：

[inline-code-attrs-start title = 'QuestionResultsAggregationResult 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 当前数据点（日期桶或页面）中值到该值出现次数的映射。 **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** 注意 - 当未指定 timeBucket 时为 null。 **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 聚合的结果总数。 **/
    total: number
    /** 计算得到的加权平均值。它是一个浮点数，通常保留两位小数或更少。 **/
    average: number
    /** 表示计算该数据时间的日期字符串，因为数据可能来自缓存。 **/
    createdAt: string
}
[inline-code-end]

以下是可用于聚合的查询参数：

[inline-code-attrs-start title = 'QuestionResultsAggregation 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以对一个或多个问题的结果进行聚合。 **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** 针对特定页面进行聚合。 **/
    urlId?: string
    /** 针对特定用户进行聚合。 **/
    userId?: string
    /** 强制立即重新计算并更新缓存。 **/
    forceRecalculate?: boolean
}
[inline-code-end]

下面是一个示例请求：

[inline-code-attrs-start title = 'QuestionResultsAggregation 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

示例响应：

[inline-code-attrs-start title = 'QuestionResultsAggregation 响应示例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsAggregation 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### 性能说明

- 对于缓存未命中的情况，聚合通常每百万条结果大约需要五秒。
- 否则，请求为常数时间。

### 缓存与费用说明

- `forceRecalculate` 指定时，费用始终为 `10`，而不是通常的 `2`。
- 如果缓存到期且数据被重新计算，在未指定 `forceRecalculate` 的情况下费用仍然是固定的 `2`。缓存到期时间取决于所聚合的数据集大小（可在 30 秒到 5 分钟之间变化）。
- 这是为了鼓励使用缓存。

---