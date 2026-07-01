## Параметри

| Име | Тип | Задължително | Описание |
|------|------|---------------|----------|
| tenantId | string | Да |  |
| id | string | Не |  |
| editKey | string = "" | Не |  |

## Отговор

Връща: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteRespOpt, httpResp) = client.deleteVote(tenantId = "my-tenant-123", id = "comment-456", editKey = "")
if voteRespOpt.isSome:
  let voteResp = voteRespOpt.get()
  discard voteResp
  discard httpResp
[inline-code-end]