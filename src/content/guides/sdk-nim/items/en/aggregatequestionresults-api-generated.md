## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | seq[string] | No |  |
| urlId | string | Yes |  |
| timeBucket | AggregateTimeBucket | No |  |
| startDate | string | No |  |
| forceRecalculate | bool | No |  |

## Response

Returns: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregateQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "q-42",
  questionIds = @["q-42", "q-43"],
  urlId = "news/article-latest-updates",
  timeBucket = AggregateTimeBucket.daily,
  startDate = "2025-01-01T00:00:00Z",
  forceRecalculate = false
)

if response.isSome:
  let aggregated = response.get()
  discard aggregated
[inline-code-end]
