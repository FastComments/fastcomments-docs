[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Тук се извършва комбинирането на резултати с коментари. Полезно е за създаване на диаграма "скорошни положителни и отрицателни коментари" за продукт, например.

Можете да търсите чрез диапазон от стойности (включително), един или повече въпроса и по начална дата (включително).

Структурата на отговора е следната:

[inline-code-attrs-start title = 'Структура на QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Ето наличните параметри на заявката за агрегиране:

[inline-code-attrs-start title = 'Структура на заявката за QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Ето примерна заявка:

[inline-code-attrs-start title = 'Пример за QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Примерен отговор:

[inline-code-attrs-start title = 'Примерен отговор за QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Бележки за кеширане и цена

- Когато е указано `forceRecalculate`, цената винаги е `10` вместо нормалните `2`.
- Ако кешът изтече и данните се преизчисляват, цената все още е константа `2`, ако `forceRecalculate` не е указано.
- Това е за да се стимулира използването на кеша.
