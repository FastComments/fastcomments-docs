## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateQuestionResultBody | UpdateQuestionResultBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateQuestionResult Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateQuestionResultBody = UpdateQuestionResultBody(result = "approved", confidence = 95, notes = "Verified by moderator", tags = @["verified", "policy-compliant"])
let (response, httpResponse) = client.updateQuestionResult(tenantId = "my-tenant-123", id = "question-456", updateQuestionResultBody = updateQuestionResultBody)
if response.isSome:
  let flagResp = response.get()
  echo "Flag update id: ", flagResp.id
[inline-code-end]
