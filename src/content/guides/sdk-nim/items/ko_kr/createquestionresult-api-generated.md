## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createQuestionResultBody | CreateQuestionResultBody | 아니오 |  |

## 응답

반환: [`Option[CreateQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result200response.nim)

## 예제

[inline-code-attrs-start title = 'createQuestionResult 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionResult(tenantId = "my-tenant-123",
  createQuestionResultBody = CreateQuestionResultBody(questionId: "q-456",
    userId: "user-789",
    correct: true,
    score: 9,
    answers: @["B", "D"]))
if response.isSome:
  let result = response.get()
  echo result
[inline-code-end]

---