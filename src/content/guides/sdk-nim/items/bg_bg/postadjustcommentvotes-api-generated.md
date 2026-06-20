## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]