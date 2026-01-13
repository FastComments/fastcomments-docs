## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Retourne : [`Option[GetQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_result200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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