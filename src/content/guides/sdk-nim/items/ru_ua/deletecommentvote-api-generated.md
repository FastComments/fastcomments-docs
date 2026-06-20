## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| voteId | string | Нет |  |
| urlId | string | Да |  |
| broadcastId | string | Нет |  |
| editKey | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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