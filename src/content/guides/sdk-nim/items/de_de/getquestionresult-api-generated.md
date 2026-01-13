## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Gibt zurück: [`Option[GetQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result200response.nim)

## Beispiel

[inline-code-attrs-start title = 'getQuestionResult Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if response.isSome:
  let result = response.get()
  echo "Received question result:"
  echo result
else:
  echo "No question result returned"
[inline-code-end]

---