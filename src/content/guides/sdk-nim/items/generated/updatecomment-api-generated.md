## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updatableCommentParams | UpdatableCommentParams | No |  |
| contextUserId | string | No |  |
| doSpamCheck | bool | No |  |
| isLive | bool | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdatableCommentParams(
  content = "Updated comment to clarify publication timeline",
  tags = @["news", "timeline"],
  isApproved = true
)

let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654321",
  updatableCommentParams = updateParams,
  contextUserId = "user-42",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let flagResult = response.get()
  echo "FlagCommentPublic_200_response received: ", flagResult
else:
  echo "No response body; HTTP status: ", httpResponse.status
[inline-code-end]
