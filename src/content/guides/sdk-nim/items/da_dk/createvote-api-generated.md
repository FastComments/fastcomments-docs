---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| direction | string | Nej |  |
| options | CreateVoteOptions | Nej |  |

## Svar

Returnerer: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Eksempel

[inline-code-attrs-start title = 'createVote Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---