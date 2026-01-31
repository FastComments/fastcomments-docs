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
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "comment-82b3f", contextUserId = "user-1024", isLive = true)
if response.isSome:
  let deleted = response.get()
  echo "Delete succeeded:", deleted
else:
  echo "Delete failed, HTTP status:", httpResponse.status
[inline-code-end]
