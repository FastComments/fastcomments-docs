## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| voteId | string | לא |  |
| urlId | string | כן |  |
| broadcastId | string | לא |  |
| editKey | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  voteId = "vote-789",
  urlId = "news/article-title",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let voteResp = response.get()
  echo "Vote delete response:", voteResp
else:
  echo "No response body, HTTP response:", httpResponse
[inline-code-end]

---