## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nein |  |
| questionIds | seq[string] | Nein |  |
| urlId | string | Ja |  |
| timeBucket | AggregateTimeBucket | Nein |  |
| startDate | string | Nein |  |
| forceRecalculate | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Beispiel

[inline-code-attrs-start title = 'aggregateQuestionResults Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregateQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/economy/budget-2025",
  timeBucket = AggregateTimeBucket(0),
  startDate = "",
  forceRecalculate = false
)

if response.isSome:
  let aggResults = response.get()
  discard aggResults
[inline-code-end]

---