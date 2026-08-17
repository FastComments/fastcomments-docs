[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Овде се врши агрегирање резултата.

Структура одговора агрегирања је следећа:

[inline-code-attrs-start title = 'Структура резултата агрегирања питања'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** A map of value to count of occurrences of that value for the current data point (date bucket or page). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Note - is null when timeBucket not specified. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** The total number of results aggregated. **/
    total: number
    /** The resulting weighted average. It is a float, usually two decimals or fewer. **/
    average: number
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
}
[inline-code-end]

Ово су параметри упита доступни за агрегирање:

[inline-code-attrs-start title = 'Структура захтева за агрегирање резултата питања'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ево примера захтева:

[inline-code-attrs-start title = 'Пример агрегирања резултата питања'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Пример одговора:

[inline-code-attrs-start title = 'Пример одговора агрегирања резултата питања'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора агрегирања резултата питања'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Напомене о перформансама

- За пропуст у кешу, агрегирања обично трају пет секунди по милиону резултата.
- У супротном, захтеви су константног времена.

### Напомене о кеширању и трошковима

- Када је `forceRecalculate` наведен, трошак је увек `10`, уместо уобичајеног `2`.
- Ако кеш истекне и подаци се поново израчунају, трошак је и даље константни `2` ако `forceRecalculate` није наведен. Кеш истиче у зависности од величине скупа података који се агрегира (може варирати између 30 секунди и 5 минута).
- Ово је да се подстакне коришћење кеша.