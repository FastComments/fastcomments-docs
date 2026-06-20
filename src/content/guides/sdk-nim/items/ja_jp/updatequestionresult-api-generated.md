## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateQuestionResultBody | UpdateQuestionResultBody | いいえ |  |

## レスポンス

返却値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'updateQuestionResult の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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