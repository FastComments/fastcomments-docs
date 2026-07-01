## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createQuestionResultBody | CreateQuestionResultBody | 否 |  |

## 回應

回傳：[`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let body = CreateQuestionResultBody()
let (optResult, httpResp) = client.createQuestionResult(tenantId = tenantId, createQuestionResultBody = body)
if optResult.isSome:
  let result = optResult.get()
  echo result
[inline-code-end]

---