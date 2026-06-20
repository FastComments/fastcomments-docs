---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| questionId | string | Não |  |
| questionIds | seq[string] | Não |  |
| urlId | string | Sim |  |
| timeBucket | AggregateTimeBucket | Não |  |
| startDate | string | Não |  |
| forceRecalculate | bool | Não |  |

## Resposta

Retorna: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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