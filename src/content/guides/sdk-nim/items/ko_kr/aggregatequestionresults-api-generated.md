## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니요 |  |
| questionIds | seq[string] | 아니요 |  |
| urlId | string | 예 |  |
| timeBucket | AggregateTimeBucket | 아니요 |  |
| startDate | string | 아니요 |  |
| forceRecalculate | bool | 아니요 |  |

## 응답

반환: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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