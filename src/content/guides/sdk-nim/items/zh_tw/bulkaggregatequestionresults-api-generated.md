## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 否 |  |
| forceRecalculate | bool | 否 |  |

## 回應

回傳：[`Option[BulkAggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results200response.nim)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (response, httpResponse) = client.bulkAggregateQuestionResults(tenantId = "my-tenant-123", bulkAggregateQuestionResultsRequest = request, forceRecalculate = false)
if response.isSome:
  let aggregated = response.get()
  echo aggregated
[inline-code-end]

---