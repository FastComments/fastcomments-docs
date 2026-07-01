## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Nej |  |
| urlId | string | Ja |  |
| broadcastId | string | Nej |  |
| options | DeleteCommentVoteOptions | Nej |  |

## Svar

Returnerer: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Eksempel

[inline-code-attrs-start title = 'deleteCommentVote Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  voteId = "vote-789",
  urlId = "news/article-title",
  broadcastId = "broadcast-001",
  options = DeleteCommentVoteOptions()
)

if response.isSome:
  let voteDelete = response.get()
[inline-code-end]