---
## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| questionId | string | Hayır |  |
| questionIds | seq[string] | Hayır |  |
| urlId | string | Evet |  |
| startDate | string | Hayır |  |
| forceRecalculate | bool | Hayır |  |
| minValue | float64 | Hayır |  |
| maxValue | float64 | Hayır |  |
| limit | float64 | Hayır |  |

## Yanıt

Döndürür: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Örnek

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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