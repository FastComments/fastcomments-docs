## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| questionId | string | Ні |  |
| questionIds | seq[string] | Ні |  |
| urlId | string | Так |  |
| startDate | string | Ні |  |
| forceRecalculate | bool | Ні |  |
| minValue | float64 | Ні |  |
| maxValue | float64 | Ні |  |
| limit | float64 | Ні |  |

## Відповідь

Повертає: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад combineCommentsWithQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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