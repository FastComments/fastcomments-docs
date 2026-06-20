## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 否 |  |
| forceRecalculate | bool | 否 |  |

## 回應

回傳: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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