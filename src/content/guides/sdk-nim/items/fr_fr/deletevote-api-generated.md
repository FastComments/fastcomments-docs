## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| editKey | string | Non |  |

## Réponse

Renvoie : [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "vote-7f3b2a", editKey = "")
if response.isSome:
  let voteDelete = response.get()
  echo "Vote deleted successfully"
else:
  echo "Failed to delete vote"
[inline-code-end]

---