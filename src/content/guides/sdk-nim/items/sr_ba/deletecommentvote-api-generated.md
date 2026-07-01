## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | No |  |
| urlId | string | Yes |  |
| broadcastId | string | No |  |
| options | DeleteCommentVoteOptions | No |  |

## Odgovor

Vraća: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Primjer

[inline-code-attrs-start title = 'deleteCommentVote Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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