---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Nee |  |
| forceRecalculate | bool | Nee |  |

## Respons

Geeft terug: [`Option[BulkAggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (response, httpResponse) = client.bulkAggregateQuestionResults(tenantId = "my-tenant-123", bulkAggregateQuestionResultsRequest = request, forceRecalculate = false)
if response.isSome:
  let aggregated = response.get()
  echo aggregated
[inline-code-end]

---