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
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-456",
  updatableCommentParams = UpdatableCommentParams(
    content: "Updated the comment to clarify the timeline",
    tags: @["clarification", "timeline"],
    isHidden: false
  ),
  contextUserId = "user-789",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let updatedComment = response.get()
  echo "Updated comment received, id: ", updatedComment.id
[inline-code-end]
