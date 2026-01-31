## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'unBlockUserFromComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "journalist-2468",
  anonUserId = ""
)

if response.isSome:
  let unblocked = response.get()
  echo unblocked
else:
  echo "Unblock failed"
[inline-code-end]
