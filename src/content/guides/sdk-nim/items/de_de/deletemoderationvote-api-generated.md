## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| voteId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteModerationVote Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerationVote(commentId = "my-tenant-123/news/article-title/comment-987", voteId = "vote-456", sso = "sso-token-abc")
if response.isSome:
  let voteResp = response.get()
  echo "Vote deleted:", voteResp
else:
  echo "Delete failed:", httpResponse
[inline-code-end]

---