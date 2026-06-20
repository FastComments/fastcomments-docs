## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| questionId | string | Ne |  |
| questionIds | seq[string] | Ne |  |
| urlId | string | Da |  |
| timeBucket | AggregateTimeBucket | Ne |  |
| startDate | string | Ne |  |
| forceRecalculate | bool | Ne |  |

## Odgovor

Vraća: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregateQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/article-title",
  timeBucket = AggregateTimeBucket(0),
  startDate = "",
  forceRecalculate = false
)

if response.isSome:
  let results = response.get()
  discard results
[inline-code-end]

---