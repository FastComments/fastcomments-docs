## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| voteId | string | Ні |  |
| urlId | string | Так |  |
| broadcastId | string | Ні |  |
| options | DeleteCommentVoteOptions | Ні |  |

## Відповідь

Повертає: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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