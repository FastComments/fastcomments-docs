## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createQuestionResultBody | CreateQuestionResultBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[CreateQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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