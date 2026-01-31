## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | No |  |

## Response

Returns: [`Option[CreateQuestionResult_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result200response.nim)

## Example

[inline-code-attrs-start title = 'createQuestionResult Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionResult(
  tenantId = "my-tenant-123",
  createQuestionResultBody = CreateQuestionResultBody(
    questionId = "quiz-2026-01",
    userId = "user-789",
    answer = "B",
    isCorrect = true,
    score = 8,
    tags = @["quiz","science"]
  )
)
if response.isSome:
  let result = response.get()
  echo "Created question result id: ", result.id
  echo "HTTP status: ", httpResponse.status
[inline-code-end]
