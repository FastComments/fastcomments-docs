## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nein |  |
| questionIds | seq[string] | Nein |  |
| urlId | string | Ja |  |
| startDate | string | Nein |  |
| forceRecalculate | bool | Nein |  |
| minValue | float64 | Nein |  |
| maxValue | float64 | Nein |  |
| limit | float64 | Nein |  |

## Antwort

Gibt zurück: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für combineCommentsWithQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/article-2026-climate-change",
  startDate = "",
  forceRecalculate = false,
  minValue = 0.0,
  maxValue = 0.0,
  limit = 0.0
)

if response.isSome:
  let combined = response.get()
  echo "Combined results received for tenant:", " my-tenant-123"
else:
  echo "No combined results returned"
[inline-code-end]

---