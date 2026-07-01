## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|-----------|
| tenantId | string | Да |  |
| options | GetQuestionResultsOptions | Не |  |

## Отговор

Връща: [`Option[GetQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results_response.nim)

## Пример

[inline-code-attrs-start title = 'getQuestionResults Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  options = GetQuestionResultsOptions()
)

if response.isSome:
  let results = response.get()
  echo results
[inline-code-end]

---