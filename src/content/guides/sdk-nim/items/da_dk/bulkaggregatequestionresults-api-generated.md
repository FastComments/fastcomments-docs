## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Nej |  |
| forceRecalculate | bool | Nej |  |

## Respons

Returnerer: [`Option[BulkAggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results200response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på bulkAggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (response, httpResponse) = client.bulkAggregateQuestionResults(tenantId = "my-tenant-123", bulkAggregateQuestionResultsRequest = request, forceRecalculate = false)
if response.isSome:
  let aggregated = response.get()
  echo aggregated
[inline-code-end]

---