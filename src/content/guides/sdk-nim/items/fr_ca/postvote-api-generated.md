## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| options | PostVoteOptions | Non |  |

## Réponse

Renvoie : [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Exemple

[inline-code-attrs-start title = 'postVote Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.postVote(
  tenantId = "my-tenant-123",
  commentId = "comment-789",
  options = default(PostVoteOptions)
)

if voteOpt.isSome:
  let vote = voteOpt.get()
[inline-code-end]