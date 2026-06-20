## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Response

Returns: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-98765", contextUserId = "user-456", isLive = true)
if response.isSome:
  let result = response.get()
  echo "DeleteCommentResult received"
else:
  echo "No result, HTTP status: ", httpResponse.status
[inline-code-end]
