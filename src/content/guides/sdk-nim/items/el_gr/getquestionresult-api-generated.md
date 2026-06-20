## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if response.isSome:
  let result = response.get()
  echo "Question result received:"
  echo result
else:
  echo "No question result returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---