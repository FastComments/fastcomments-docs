---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| voteId | string | 否 |  |
| urlId | string | 是 |  |
| broadcastId | string | 否 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## 示例

[inline-code-attrs-start title = 'deleteCommentVote 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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