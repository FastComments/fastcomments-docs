## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateQuestionConfig Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateQuestionConfigBody(question = "Was this article helpful?", enabled = true, options = @["Yes", "No", "Not sure"])
let (response, httpResponse) = client.updateQuestionConfig(tenantId = "my-tenant-123", id = "question-456", updateQuestionConfigBody = updateBody)
if response.isSome:
  let cfg = response.get()
  echo "Update successful for tenant: ", "my-tenant-123"
  discard cfg
[inline-code-end]
