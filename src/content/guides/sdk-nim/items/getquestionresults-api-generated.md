## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetQuestionResults_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_results200response.nim)

## Example

[inline-code-attrs-start title = 'getQuestionResults Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionResults(
  tenantId = "my-tenant-123",
  urlId = "news/today-top-story",
  userId = "",
  startDate = "",
  questionId = "",
  questionIds = "",
  skip = 0.0
)
if response.isSome:
  let results = response.get()
  echo "Fetched question results."
[inline-code-end]
