## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'blockUserFromComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654321",
  blockFromCommentParams = BlockFromCommentParams(),
  userId = "user-42",
  anonUserId = ""
)
if response.isSome:
  let blocked = response.get()
  discard blocked
[inline-code-end]
