## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| questionId | string | Όχι |  |
| questionIds | seq[string] | Όχι |  |
| urlId | string | Ναι |  |
| startDate | string | Όχι |  |
| forceRecalculate | bool | Όχι |  |
| minValue | float64 | Όχι |  |
| maxValue | float64 | Όχι |  |
| limit | float64 | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[CombineCommentsWithQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_comments_with_question_results200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "q-9876",
  questionIds = @["q-9876", "q-9877"],
  urlId = "news/article-title",
  startDate = "2025-01-01T00:00:00Z",
  forceRecalculate = false,
  minValue = 1.0,
  maxValue = 5.0,
  limit = 100.0
)

if response.isSome:
  let combined = response.get()
  discard combined
[inline-code-end]