## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Example

[inline-code-attrs-start title = 'pinComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.pinComment(tenantId = "my-tenant-123", commentId = "cmt-98765", broadcastId = "", sso = "")
if response.isSome:
  let pinnedResp = response.get()
  echo "Pin status updated for comment cmt-98765"
else:
  echo "No response received"
[inline-code-end]
