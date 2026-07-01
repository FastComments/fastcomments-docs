## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | AggregateQuestionResultsOptions | No |  |

## Response

Returns: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = AggregateQuestionResultsOptions()
let (aggResultOpt, httpResp) = client.aggregateQuestionResults(tenantId = "my-tenant-123", options = opts)
if aggResultOpt.isSome:
  let aggResult = aggResultOpt.get()
[inline-code-end]
