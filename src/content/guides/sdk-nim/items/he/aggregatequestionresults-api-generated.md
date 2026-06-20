## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| questionId | string | לא |  |
| questionIds | seq[string] | לא |  |
| urlId | string | כן |  |
| timeBucket | AggregateTimeBucket | לא |  |
| startDate | string | לא |  |
| forceRecalculate | bool | לא |  |

## תגובה

מחזיר: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregateQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/article-title",
  timeBucket = AggregateTimeBucket(0),
  startDate = "",
  forceRecalculate = false
)

if response.isSome:
  let results = response.get()
  discard results
[inline-code-end]