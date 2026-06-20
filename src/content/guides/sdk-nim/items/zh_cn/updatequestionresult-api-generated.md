## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionResult(
  tenantId = "my-tenant-123",
  id = "question-result-456",
  updateQuestionResultBody = UpdateQuestionResultBody(
    questionId: "q-789",
    userId: "user-42",
    score: 92,
    passed: true,
    tags: @["quiz", "math"]
  )
)
if response.isSome:
  let apiResp = response.get()
  echo "Question result updated successfully"
else:
  echo "No response body; HTTP status: ", httpResponse.status.code
[inline-code-end]

---