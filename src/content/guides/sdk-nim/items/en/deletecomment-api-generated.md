## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Response

Returns: [`Option[DeleteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment200response.nim)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "comment-9876", contextUserId = "user-42", isLive = true)
if response.isSome:
  let deletedComment = response.get()
  discard deletedComment
else:
  echo "Delete failed, status: ", httpResponse.status
[inline-code-end]
