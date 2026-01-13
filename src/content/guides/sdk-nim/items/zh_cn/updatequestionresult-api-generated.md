## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateQuestionResultBody | UpdateQuestionResultBody | No |  |

## 响应

返回: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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