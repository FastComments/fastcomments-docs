[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Это место, где выполняется агрегирование результатов.

Структура ответа агрегации выглядит следующим образом:

[inline-code-attrs-start title = 'Структура QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Карта значения к количеству его вхождений для текущей точки данных (датовый бакет или страница). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Примечание — null, когда timeBucket не указан. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Общее количество агрегированных результатов. **/
    total: number
    /** Полученное взвешенное среднее. Это число с плавающей точкой, обычно с двумя знаками после запятой или меньше. **/
    average: number
    /** Строка даты, указывающая, когда эти данные были рассчитаны, поскольку они могут браться из кэша. **/
    createdAt: string
}
[inline-code-end]

Ниже перечислены параметры запроса, доступные для агрегации:

[inline-code-attrs-start title = 'Структура запроса QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Можно агрегировать результаты для одного или нескольких вопросов. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Агрегировать для конкретной страницы. **/
    urlId?: string
    /** Агрегировать для конкретного пользователя. **/
    userId?: string
    /** Принудительно пересчитать сейчас и обновить кэш. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Пример запроса:

[inline-code-attrs-start title = 'Пример QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Включено при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включено при ошибке. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Примечания по производительности

- При промахе кэша агрегация обычно занимает пять секунд на миллион результатов.
- В противном случае запросы выполняются за постоянное время.

### Примечания по кэшированию и стоимости

- Когда указан `forceRecalculate`, стоимость всегда равна `10`, вместо обычных `2`.
- Если кэш истекает и данные пересчитываются, стоимость по-прежнему составляет постоянные `2`, если `forceRecalculate` не указан. Срок жизни кэша зависит от объема агрегируемого набора данных (может варьироваться от 30 секунд до 5 минут).
- Это сделано для стимулирования использования кэша.

---