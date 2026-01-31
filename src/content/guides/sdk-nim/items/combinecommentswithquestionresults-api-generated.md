## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | seq[string] | No |  |
| urlId | string | Yes |  |
| startDate | string | No |  |
| forceRecalculate | bool | No |  |
| minValue | float64 | No |  |
| maxValue | float64 | No |  |
| limit | float64 | No |  |

## Response

Returns: [`Option[CombineCommentsWithQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_comments_with_question_results200response.nim)

## Example

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  questionId = "q-67890",
  questionIds = @["q-67890", "q-11111"],
  urlId = "news/article-title",
  startDate = "2025-01-01T00:00:00Z",
  forceRecalculate = true,
  minValue = 0.0,
  maxValue = 5.0,
  limit = 100.0
)

if response.isSome:
  let combined = response.get()
  echo combined
[inline-code-end]
