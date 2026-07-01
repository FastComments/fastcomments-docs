## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| skip | float64 | Non |  |

## Réponse

Retourne : [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerators'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorsOpt, httpResp) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if moderatorsOpt.isSome:
  let moderators = moderatorsOpt.get()
  echo moderators
[inline-code-end]