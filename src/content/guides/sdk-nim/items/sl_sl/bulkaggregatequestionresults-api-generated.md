## Parametri

| Ime | Tip | Potrebno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | No |  |
| forceRecalculate | bool | No |  |

## Odgovor

Vrne: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## Primer

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (maybeResult, httpResp) = client.bulkAggregateQuestionResults(
  tenantId = "my-tenant-123",
  bulkAggregateQuestionResultsRequest = request,
  forceRecalculate = false)

if maybeResult.isSome:
  let result = maybeResult.get()
[inline-code-end]