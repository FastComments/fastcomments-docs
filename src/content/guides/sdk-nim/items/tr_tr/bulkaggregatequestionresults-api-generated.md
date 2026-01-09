## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Hayır |  |
| forceRecalculate | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[BulkAggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results200response.nim)

## Örnek

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (response, httpResponse) = client.bulkAggregateQuestionResults(tenantId = "my-tenant-123", bulkAggregateQuestionResultsRequest = request, forceRecalculate = false)
if response.isSome:
  let aggregated = response.get()
  echo aggregated
[inline-code-end]

---