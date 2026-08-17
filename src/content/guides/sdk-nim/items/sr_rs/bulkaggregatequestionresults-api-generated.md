## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Не |  |
| forceRecalculate | bool | Не |  |

## Одговор

Returns: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## Пример

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (maybeResult, httpResp) = client.bulkAggregateQuestionResults(
  tenantId = "my-tenant-123",
  bulkAggregateQuestionResultsRequest = request,
  forceRecalculate = false)

if maybeResult.isSome:
  let result = maybeResult.get()
[inline-code-end]