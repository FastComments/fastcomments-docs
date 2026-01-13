## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| direction | string | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## Beispiel

[inline-code-attrs-start title = 'createVote Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  direction = "up",
  userId = "user-42",
  anonUserId = ""
)
if response.isSome:
  let vote = response.get()
  echo "Vote recorded: ", $vote
else:
  echo "Vote not created, HTTP response: ", $httpResponse
[inline-code-end]

---