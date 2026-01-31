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
var updatableCommentParams = UpdatableCommentParams(
  content = "Updated the timeline after editorial review.",
  tags = @["news","correction"],
  isEdited = true
)

let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654321",
  updatableCommentParams = updatableCommentParams,
  contextUserId = "user-42",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let updated = response.get()
  echo "Updated comment:", updated
else:
  echo "Update failed, HTTP status:", httpResponse.status
[inline-code-end]
