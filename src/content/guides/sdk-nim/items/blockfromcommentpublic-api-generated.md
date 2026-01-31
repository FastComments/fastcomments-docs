## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'blockFromCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)
if response.isSome:
  let blockResp = response.get()
  echo "BlockFromCommentPublic succeeded: ", $blockResp
else:
  echo "No block response, HTTP status: ", $httpResponse.status
[inline-code-end]
