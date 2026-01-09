[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Овдје се врши агрегирање резултата.

Структура одговора агрегирања је следећа:

[inline-code-attrs-start title = 'Структура QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Мапа вриједности на број појављивања те вриједности за тренутну тачку података (датумски сегмент или страница). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Напомена - је null када timeBucket није наведен. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Укупни број агрегираних резултата. **/
    total: number
    /** Резултујући важећи просјек. То је float, обично са највише двије децимале. **/
    average: number
    /** Стринг датума који представља када су ови подаци израчунати, јер могу доћи из кеша. **/
    createdAt: string
}
[inline-code-end]

Ово су параметри упита доступни за агрегирање:

[inline-code-attrs-start title = 'Структура захтјева QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Можете агрегирати резултате за једно или више питања. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Агрегирати за одређену страницу. **/
    urlId?: string
    /** Агрегирати за одређеног корисника. **/
    userId?: string
    /** Принуди поновно израчунавање сада и ажурирај кеш. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Ево примера захтјева:

[inline-code-attrs-start title = 'Пример QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Пример одговора:

[inline-code-attrs-start title = 'Пример одговора QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Напомене о перформансама

- Ако нема података у кешу (cache miss), агрегирање обично траје пет секунди по милиону резултата.
- Иначе, захтјеви су константног времена.

### Напомене о кеширању и трошковима

- Када је `forceRecalculate` наведен, цијена је увијек `10`, умјесто уобичајених `2`.
- Ако кеш истекне и подаци се поново израчунају, цијена је и даље константна `2` ако `forceRecalculate` није наведен. Кеш истиче у зависности од величине скупа података који се агрегира (може варирати између 30 секунди и 5 минута).
- Ово је да би се потакло коришћење кеша.

---