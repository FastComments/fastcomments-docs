## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| questionId | string | Não |  |
| questionIds | seq[string] | Não |  |
| urlId | string | Sim |  |
| startDate | string | Não |  |
| forceRecalculate | bool | Não |  |
| minValue | float64 | Não |  |
| maxValue | float64 | Não |  |
| limit | float64 | Não |  |

## Resposta

Retorna: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de combineCommentsWithQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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