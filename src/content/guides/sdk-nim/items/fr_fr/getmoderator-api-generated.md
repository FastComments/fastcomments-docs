## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie : [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-456")
if response.isSome:
  let moderator = response.get()
  echo moderator
else:
  echo "Moderator not found, HTTP status: ", $httpResponse.status
[inline-code-end]