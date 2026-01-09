## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 아니오 |  |
| forceRecalculate | bool | 아니오 |  |

## 응답

반환: [`Option[BulkAggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_aggregate_question_results200response.nim)

## 예제

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let request = BulkAggregateQuestionResultsRequest()
let (response, httpResponse) = client.bulkAggregateQuestionResults(tenantId = "my-tenant-123", bulkAggregateQuestionResultsRequest = request, forceRecalculate = false)
if response.isSome:
  let aggregated = response.get()
  echo aggregated
[inline-code-end]

---