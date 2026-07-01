## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| voteId | string | Non |  |
| urlId | string | Oui |  |
| broadcastId | string | Non |  |
| options | DeleteCommentVoteOptions | Non |  |

## Response

Retourne : [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteCommentVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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