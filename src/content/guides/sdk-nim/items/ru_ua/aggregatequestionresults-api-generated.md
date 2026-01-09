## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| questionId | string | Нет |  |
| questionIds | seq[string] | Нет |  |
| urlId | string | Да |  |
| timeBucket | AggregateTimeBucket | Нет |  |
| startDate | string | Нет |  |
| forceRecalculate | bool | Нет |  |

## Ответ

Возвращает: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregateQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "",
  questionIds = @[],
  urlId = "news/economy/budget-2025",
  timeBucket = AggregateTimeBucket(0),
  startDate = "",
  forceRecalculate = false
)

if response.isSome:
  let aggResults = response.get()
  discard aggResults
[inline-code-end]

---