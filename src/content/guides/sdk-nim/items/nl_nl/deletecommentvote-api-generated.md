## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Nee |  |
| urlId | string | Ja |  |
| broadcastId | string | Nee |  |
| editKey | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'deleteCommentVote Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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