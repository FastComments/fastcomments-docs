## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createQuestionResultBody | CreateQuestionResultBody | Nein |  |

## Antwort

Gibt zurück: [`Option[CreateQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result200response.nim)

## Beispiel

[inline-code-attrs-start title = 'createQuestionResult Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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