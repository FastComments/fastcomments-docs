## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostVoteOptions | No |  |

## Απόκριση

Επιστρέφει: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.postVote(
  tenantId = "my-tenant-123",
  commentId = "comment-789",
  options = default(PostVoteOptions)
)

if voteOpt.isSome:
  let vote = voteOpt.get()
[inline-code-end]