## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| voteId | string | Ne |  |
| urlId | string | Da |  |
| broadcastId | string | Ne |  |
| editKey | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Primer

[inline-code-attrs-start title = 'deleteCommentVote Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-789",
  voteId = "",
  urlId = "news/breaking-story-2025",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Vote removed for comment cmt-789"
else:
  echo "No response body returned"
[inline-code-end]

---