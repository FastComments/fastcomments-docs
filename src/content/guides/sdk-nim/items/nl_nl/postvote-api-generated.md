## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostVoteOptions | Nee |  |

## Respons

Retourneert: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postVote Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.postVote(
  tenantId = "my-tenant-123",
  commentId = "comment-789",
  options = default(PostVoteOptions)
)

if voteOpt.isSome:
  let vote = voteOpt.get()
[inline-code-end]

---