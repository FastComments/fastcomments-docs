## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'unBlockCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(reason = "", blockDurationMinutes = 0, notifyUser = false),
  sso = ""
)
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]
