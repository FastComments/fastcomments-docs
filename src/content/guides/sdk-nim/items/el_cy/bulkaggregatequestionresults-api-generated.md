## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Όχι |  |
| forceRecalculate | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.bulkAggregateQuestionResults(
  tenantId = "my-tenant-123",
  bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(),
  forceRecalculate = false
)

if response.isSome:
  let aggregated = response.get()
  echo "Aggregated question results received"
[inline-code-end]

---