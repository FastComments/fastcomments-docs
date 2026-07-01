## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

返回: [`Option[GetQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result_response.nim)

## 範例

[inline-code-attrs-start title = 'getQuestionResult 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResult, httpResp) = client.getQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if optResult.isSome:
  let result = optResult.get()
  discard result
[inline-code-end]