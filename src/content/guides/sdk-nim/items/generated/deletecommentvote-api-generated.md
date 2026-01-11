## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | No |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Example

[inline-code-attrs-start title = 'deleteCommentVote Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  voteId = "",
  urlId = "news/2025/fastcomments-case-study",
  broadcastId = "",
  editKey = "",
  sso = ""
)

if response.isSome:
  let deleted = response.get()
  echo "DeleteCommentVote succeeded: ", $deleted
else:
  echo "DeleteCommentVote failed, HTTP response: ", $httpResponse
[inline-code-end]
