## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userId | string | Nein |  |
| startDate | string | Nein |  |
| questionId | string | Nein |  |
| questionIds | string | Nein |  |
| skip | float64 | Nein |  |

## Antwort

Gibt zurück: [`Option[GetQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getQuestionResults Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  urlId = "news/2026/election-analysis",
  userId = "user-42",
  startDate = "2026-06-01T00:00:00Z",
  questionId = "q-6789",
  questionIds = @["q-6789", "q-6790"],
  skip = 0.0
)
if response.isSome:
  let results = response.get()
  echo "Received question results"
else:
  echo "No results returned"
[inline-code-end]

---