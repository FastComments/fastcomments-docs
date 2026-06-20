## Параметри

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| userId | string | Ні |  |
| startDate | string | Ні |  |
| questionId | string | Ні |  |
| questionIds | string | Ні |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetQuestionResultsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results_response.nim)

## Приклад

[inline-code-attrs-start title = 'getQuestionResults Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  urlId = "news/2026/election-analysis",
  userId = "user-42",
  startDate = "2026-06-01T00:00:00Z",
  questionId = "q-6789",
  questionIds = @["q-6789", "q-6790"],
  skip = 0.0
)
if response.isSome:
  let results = response.get()
  echo "Received question results"
else:
  echo "No results returned"
[inline-code-end]

---