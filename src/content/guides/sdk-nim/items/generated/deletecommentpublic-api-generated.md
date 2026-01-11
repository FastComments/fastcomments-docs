## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[DeleteCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'deleteCommentPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-4f2b9a",
  broadcastId = "",
  editKey = "",
  sso = ""
)

if response.isSome:
  let deleted = response.get()
  echo "Comment deleted successfully"
else:
  echo "Delete request failed"
[inline-code-end]
