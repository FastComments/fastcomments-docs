## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | No |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[SetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text200response.nim)

## Example

[inline-code-attrs-start title = 'setCommentText Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentUpdate = CommentTextUpdateRequest(text = "Thanks for the update! Clarifying my point.")
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  broadcastId = "broadcast-789",
  commentTextUpdateRequest = commentUpdate,
  editKey = "edit-key-abc-123",
  sso = "sso-token-xyz"
)
if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse
[inline-code-end]
