## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[PinComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pin_comment200response.nim)

## Example

[inline-code-attrs-start title = 'unPinComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unPinComment(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", sso = "")
if response.isSome:
  let pinResponse = response.get()
  discard pinResponse
  echo "Unpin successful for comment cmt-987654321"
else:
  echo "Failed to unpin comment"
[inline-code-end]
