## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nee |  |
| questionIds | seq[string] | Nee |  |
| urlId | string | Ja |  |
| timeBucket | AggregateTimeBucket | Nee |  |
| startDate | string | Nee |  |
| forceRecalculate | bool | Nee |  |

## Antwoord

Retourneert: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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