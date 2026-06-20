## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
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

## Απόκριση

Επιστρέφει: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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