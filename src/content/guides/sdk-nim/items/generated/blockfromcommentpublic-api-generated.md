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
  commentId = "news/article-2026-01-12-comment-789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(
    reason = "harassment",
    durationMinutes = 1440,
    notifyUsers = false,
    tags = @["abuse","spam"]
  ),
  sso = ""
)
if response.isSome:
  let blockResult = response.get()
  discard blockResult
[inline-code-end]
