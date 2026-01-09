## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Nein |  |
| voteBodyParams | VoteBodyParams | Nein |  |
| sessionId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Beispiel

[inline-code-attrs-start title = 'voteComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let voteBody = VoteBodyParams()
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  urlId = "news/2025/fastcomments-integration",
  broadcastId = "",
  voteBodyParams = voteBody,
  sessionId = "",
  sso = ""
)
if response.isSome:
  let voteResp = response.get()
  echo "Vote recorded for comment cmt-456789"
else:
  echo "Failed to record vote"
[inline-code-end]

---