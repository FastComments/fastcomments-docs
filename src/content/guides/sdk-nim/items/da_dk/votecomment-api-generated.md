## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Nej |  |
| voteBodyParams | VoteBodyParams | Nej |  |
| sessionId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Eksempel

[inline-code-attrs-start title = 'voteComment-eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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