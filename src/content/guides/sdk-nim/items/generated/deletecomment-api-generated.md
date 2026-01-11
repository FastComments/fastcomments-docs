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
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "", contextUserId = "", isLive = false)
if response.isSome:
  let deleted = response.get()
  echo "DeleteComment response received"
else:
  echo "No DeleteComment response body received"
[inline-code-end]
