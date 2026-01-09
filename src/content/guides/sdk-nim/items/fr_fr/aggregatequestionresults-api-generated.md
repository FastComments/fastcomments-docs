## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| questionId | string | Non |  |
| questionIds | seq[string] | Non |  |
| urlId | string | Oui |  |
| timeBucket | AggregateTimeBucket | Non |  |
| startDate | string | Non |  |
| forceRecalculate | bool | Non |  |

## Réponse

Renvoie : [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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