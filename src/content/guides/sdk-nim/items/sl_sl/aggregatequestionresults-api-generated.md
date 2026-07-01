## Parametri

| Ime | Tip | Potrebno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | AggregateQuestionResultsOptions | Ne |  |

## Odziv

Vrne: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## Primer

[inline-code-attrs-start title = 'aggregateQuestionResults Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = AggregateQuestionResultsOptions()
let (aggResultOpt, httpResp) = client.aggregateQuestionResults(tenantId = "my-tenant-123", options = opts)
if aggResultOpt.isSome:
  let aggResult = aggResultOpt.get()
[inline-code-end]