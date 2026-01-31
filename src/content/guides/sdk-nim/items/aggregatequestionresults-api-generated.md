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
  questionId = "q-789",
  questionIds = @["q-123", "q-456"],
  urlId = "news/2026-election-analysis",
  timeBucket = AggregateTimeBucket.hour,
  startDate = "2025-12-01T00:00:00Z",
  forceRecalculate = false
)

if response.isSome:
  let results = response.get()
  echo "Aggregated question results received: ", $results
[inline-code-end]
