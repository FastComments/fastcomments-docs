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
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  broadcastId = "bcast-2026-01",
  commentTextUpdateRequest = CommentTextUpdateRequest(text = "Updated comment text for clarity and formatting"),
  editKey = "",
  sso = ""
)

if response.isSome:
  let updated = response.get()
  echo "Comment updated:", updated
else:
  echo "Update failed, status:", httpResponse.status
[inline-code-end]
