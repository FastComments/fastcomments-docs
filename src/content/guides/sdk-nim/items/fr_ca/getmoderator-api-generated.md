## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie : [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorOpt, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "moderator-456")
if moderatorOpt.isSome:
  let moderator = moderatorOpt.get()
  discard moderator
[inline-code-end]