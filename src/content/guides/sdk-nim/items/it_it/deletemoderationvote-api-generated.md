## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| voteId | string | No |  |
| options | DeleteModerationVoteOptions | No |  |

## Risposta

Restituisce: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Esempio

[inline-code-attrs-start title = 'deleteModerationVote Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---