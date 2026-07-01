## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | No |  |
| options | DeleteModerationVoteOptions | No |  |

## Ответ

Возвращает: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteModerationVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteModerationVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  voteId = "vote-abc123",
  options = DeleteModerationVoteOptions()
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]