## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| questionId | string | Hayır |  |
| questionIds | seq[string] | Hayır |  |
| urlId | string | Evet |  |
| timeBucket | AggregateTimeBucket | Hayır |  |
| startDate | string | Hayır |  |
| forceRecalculate | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[AggregateQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_question_results200response.nim)

## Örnek

[inline-code-attrs-start title = 'aggregateQuestionResults Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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