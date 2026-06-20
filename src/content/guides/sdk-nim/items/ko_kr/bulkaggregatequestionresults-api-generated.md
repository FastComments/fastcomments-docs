## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 아니요 |  |
| forceRecalculate | bool | 아니요 |  |

## 응답

반환: [`Option[BulkAggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results_response.nim)

## 예제

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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