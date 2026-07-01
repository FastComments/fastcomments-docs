## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| direction | string | Ne |  |
| options | CreateVoteOptions | Ne |  |

## Odgovor

Vraća: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Primer

[inline-code-attrs-start title = 'createVote Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  direction = "up",
  options = CreateVoteOptions()
)

if voteOpt.isSome:
  let vote = voteOpt.get()
  echo vote
[inline-code-end]