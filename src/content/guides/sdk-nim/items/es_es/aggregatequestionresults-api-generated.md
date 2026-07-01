## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| options | AggregateQuestionResultsOptions | No |  |

## Respuesta

Devuelve: [`Option[AggregateQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'aggregateQuestionResults Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = AggregateQuestionResultsOptions()
let (aggResultOpt, httpResp) = client.aggregateQuestionResults(tenantId = "my-tenant-123", options = opts)
if aggResultOpt.isSome:
  let aggResult = aggResultOpt.get()
[inline-code-end]