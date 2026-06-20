## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| voteId | string | Ні |  |
| urlId | string | Так |  |
| broadcastId | string | Ні |  |
| editKey | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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