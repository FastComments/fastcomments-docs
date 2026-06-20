## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| editKey | string | Nein |  |

## Antwort

Gibt zurück: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteVote Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "vote-7f3b2a", editKey = "")
if response.isSome:
  let voteDelete = response.get()
  echo "Vote deleted successfully"
else:
  echo "Failed to delete vote"
[inline-code-end]

---