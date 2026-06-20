## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionResultBody | CreateQuestionResultBody | いいえ |  |

## レスポンス

戻り値: [`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## 例

[inline-code-attrs-start title = 'createQuestionResult の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionResult(
  tenantId = "my-tenant-123",
  createQuestionResultBody = CreateQuestionResultBody(
    questionId = "q-2026-001",
    userId = "user-42",
    correct = true,
    score = 95,
    tags = @["news","reader-question"]
  )
)
if response.isSome:
  let result = response.get()
  echo "Created question result id: ", result.id
  echo "HTTP status: ", httpResponse.status.code
[inline-code-end]

---