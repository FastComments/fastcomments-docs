## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| voteId | string | 아니오 |  |
| urlId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## 예제

[inline-code-attrs-start title = 'deleteCommentVote 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-789",
  voteId = "",
  urlId = "news/breaking-story-2025",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Vote removed for comment cmt-789"
else:
  echo "No response body returned"
[inline-code-end]

---