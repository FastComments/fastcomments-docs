## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | CombineCommentsWithQuestionResultsOptions | Ne |  |

## Odgovor

Vraća: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Primer

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (combineOpt, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  options = default(CombineCommentsWithQuestionResultsOptions)
)

if combineOpt.isSome:
  let combineResult = combineOpt.get()
[inline-code-end]