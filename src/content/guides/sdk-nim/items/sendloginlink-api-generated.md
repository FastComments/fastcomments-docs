## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| redirectURL | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'sendLoginLink Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "my-tenant-123", id = "", redirectURL = "")
if response.isSome:
  let flagResp = response.get()
  echo "Login link sent successfully for tenant my-tenant-123"
else:
  echo "Failed to send login link"
[inline-code-end]
