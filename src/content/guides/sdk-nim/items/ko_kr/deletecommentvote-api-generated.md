## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| voteId | string | 아니오 |  |
| urlId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| options | DeleteCommentVoteOptions | 아니오 |  |

## 응답

Returns: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteCommentVote 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  voteId = "vote-789",
  urlId = "news/article-title",
  broadcastId = "broadcast-001",
  options = DeleteCommentVoteOptions()
)

if response.isSome:
  let voteDelete = response.get()
[inline-code-end]

---