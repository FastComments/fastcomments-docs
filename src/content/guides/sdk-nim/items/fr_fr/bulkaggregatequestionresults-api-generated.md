## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Non |  |
| forceRecalculate | bool | Non |  |

## Réponse

Retourne: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de bulkAggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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