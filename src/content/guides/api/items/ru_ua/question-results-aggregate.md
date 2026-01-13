[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Здесь выполняется агрегирование результатов.

Структура ответа агрегирования выглядит следующим образом:

[inline-code-attrs-start title = 'Структура QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Карта значения к количеству появлений этого значения для текущей точки данных (дата-бакет или страница). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Примечание — имеет значение null, когда timeBucket не указан. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Общее число агрегированных результатов. **/
    total: number
    /** Получившееся взвешенное среднее. Это число с плавающей запятой, обычно с двумя знаками после запятой или меньше. **/
    average: number
    /** Строка даты, указывающая, когда эти данные были вычислены, так как они могли быть взяты из кэша. **/
    createdAt: string
}
[inline-code-end]

Ниже перечислены параметры запроса, доступные для агрегирования:

[inline-code-attrs-start title = 'Структура запроса QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Вы можете агрегировать результаты для одного или нескольких вопросов. **/
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

Пример запроса:

[inline-code-attrs-start title = 'Пример запроса QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Пример ответа:

[inline-code-attrs-start title = 'Пример ответа QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Примечания по производительности

- При отсутствии кэша агрегирование обычно занимает около пяти секунд на миллион результатов.
- В противном случае запросы выполняются за константное время.

### Примечания по кэшированию и стоимости

- Когда `forceRecalculate` указан, стоимость всегда равна `10`, вместо обычных `2`.
- Если кэш истёк и данные пересчитываются, стоимость по-прежнему составляет константные `2`, если `forceRecalculate` не указан. Срок жизни кэша зависит от объёма агрегируемого набора данных (может варьироваться от 30 секунд до 5 минут).
- Это сделано для поощрения использования кэша.