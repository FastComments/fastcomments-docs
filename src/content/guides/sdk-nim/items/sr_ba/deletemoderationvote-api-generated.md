## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| voteId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Пример

[inline-code-attrs-start title = 'deleteModerationVote Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerationVote(commentId = "my-tenant-123/news/article-title/comment-987", voteId = "vote-456", sso = "sso-token-abc")
if response.isSome:
  let voteResp = response.get()
  echo "Vote deleted:", voteResp
else:
  echo "Delete failed:", httpResponse
[inline-code-end]

---