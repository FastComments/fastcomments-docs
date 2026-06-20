## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Example

[inline-code-attrs-start title = 'unBlockCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", publicBlockFromCommentParams = PublicBlockFromCommentParams(), sso = "")
if response.isSome:
  let unblockResult = response.get()
  discard unblockResult
else:
  discard httpResponse
[inline-code-end]
