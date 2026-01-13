---
## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 否 |  |

## 回應

回傳：[`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'updateQuestionResult 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionResult(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionResultBody = UpdateQuestionResultBody(
    result = "approved",
    reviewerId = "moderator-42",
    notes = "Valid question, no action required",
    isSpam = false
  )
)
if response.isSome:
  let flagResponse = response.get()
  discard flagResponse
[inline-code-end]

---