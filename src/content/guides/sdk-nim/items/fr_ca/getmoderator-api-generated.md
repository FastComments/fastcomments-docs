## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Retourne : [`Option[GetModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-98765")
if response.isSome:
  let moderator = response.get()
  discard moderator
[inline-code-end]

---