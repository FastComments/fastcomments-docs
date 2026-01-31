## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## Example

[inline-code-attrs-start title = 'lockComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  sso = ""
)

if response.isSome:
  let lockResult = response.get()
  echo "Locked comment: ", "cmt-456789"
else:
  echo "Failed to lock comment"
[inline-code-end]
