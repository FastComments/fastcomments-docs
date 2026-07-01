## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| editKey | string = "" | No |  |

## Resposta

Retorna: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteRespOpt, httpResp) = client.deleteVote(tenantId = "my-tenant-123", id = "comment-456", editKey = "")
if voteRespOpt.isSome:
  let voteResp = voteRespOpt.get()
  discard voteResp
  discard httpResp
[inline-code-end]

---