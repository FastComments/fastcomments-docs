## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createQuestionResultBody | CreateQuestionResultBody | Não |  |

## Resposta

Retorna: [`Option[CreateQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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