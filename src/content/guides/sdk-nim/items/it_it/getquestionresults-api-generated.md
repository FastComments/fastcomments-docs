## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | GetQuestionResultsOptions | No |  |

## Risposta

Restituisce: [`Option[GetQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  options = GetQuestionResultsOptions()
)

if response.isSome:
  let results = response.get()
  echo results
[inline-code-end]