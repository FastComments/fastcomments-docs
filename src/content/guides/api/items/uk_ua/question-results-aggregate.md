[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Тут відбувається агрегація результатів.

Структура відповіді агрегації така:

[inline-code-attrs-start title = 'Структура QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Мапа значення до кількості появ цього значення для поточної точки даних (датовий бакет або сторінка). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Примітка — null, коли timeBucket не вказано. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Загальна кількість агрегованих результатів. **/
    total: number
    /** Результуюче зважене середнє. Це число з плаваючою комою, зазвичай з двома або менше знаками після коми. **/
    average: number
    /** Рядок дати, що позначає, коли ці дані були обчислені, оскільки вони могли бути взяті з кешу. **/
    createdAt: string
}
[inline-code-end]

Нижче наведено параметри запиту, доступні для агрегації:

[inline-code-attrs-start title = 'Структура запиту QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Можна агрегувати результати для одного або кількох питань. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Агрегувати для певної сторінки. **/
    urlId?: string
    /** Агрегувати для конкретного користувача. **/
    userId?: string
    /** Примусово перерахувати зараз і оновити кеш. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Ось приклад запиту:

[inline-code-attrs-start title = 'Приклад QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Приклад відповіді:

[inline-code-attrs-start title = 'Приклад відповіді QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у разі помилки. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Примітки щодо продуктивності

- У разі промаху кеша (cache miss) агрегація зазвичай займає п'ять секунд на мільйон результатів.
- В іншому випадку запити виконуються за константний час.

### Примітки щодо кешування та вартості

- Коли зазначено `forceRecalculate`, вартість завжди `10` замість звичайних `2`.
- Якщо кеш минув термін дії і дані перераховуються, вартість все одно залишається константою `2`, якщо `forceRecalculate` не вказано. Термін дії кешу залежить від розміру агрегованого набору даних (може варіюватися від 30 секунд до 5 хвилин).
- Це зроблено для заохочення використання кешу.