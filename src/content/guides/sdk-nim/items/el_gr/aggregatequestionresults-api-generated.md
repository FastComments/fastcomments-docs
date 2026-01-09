## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| questionId | string | Όχι |  |
| questionIds | seq[string] | Όχι |  |
| urlId | string | Ναι |  |
| timeBucket | AggregateTimeBucket | Όχι |  |
| startDate | string | Όχι |  |
| forceRecalculate | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregateQuestionResults'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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