[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Тут відбувається комбінування результатів із коментарями. Корисно, наприклад, для створення діаграми «останні позитивні та негативні коментарі» для продукту.

Ви можете шукати за діапазоном значень (включно), за одним або кількома питаннями, та за початковою датою (включно).

Структура відповіді виглядає так:

[inline-code-attrs-start title = 'Структура QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Рядок дати, що вказує коли ці дані були обчислені, оскільки вони могли бути взяті з кешу. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Ось доступні параметри запиту для агрегації:

[inline-code-attrs-start title = 'Структура запиту QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Ви можете агрегувати результати для одного або кількох питань. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Ось приклад запиту:

[inline-code-attrs-start title = 'Приклад запиту QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Приклад відповіді:

[inline-code-attrs-start title = 'Приклад відповіді QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура відповіді QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у разі помилки. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Примітки щодо кешування та вартості

- Якщо вказано `forceRecalculate`, вартість завжди становить `10`, замість звичних `2`.
- Якщо термін дії кешу минув і дані перераховуються, вартість все ще залишається постійною `2`, якщо `forceRecalculate` не вказано.
- Це зроблено, щоб стимулювати використання кешу.