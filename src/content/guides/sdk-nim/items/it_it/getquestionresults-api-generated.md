## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | float64 | No |  |

## Risposta

Restituisce: [`Option[GetQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  urlId = "news/politics/election-2024",
  userId = "user-9876",
  startDate = "2024-01-01T00:00:00Z",
  questionId = "q-user-satisfaction",
  questionIds = "q-user-satisfaction,q-engagement",
  skip = 0.0
)

if response.isSome:
  let results = response.get()
  echo "Got question results: ", $results
else:
  echo "No results, HTTP status: ", httpResponse.status
[inline-code-end]

---