---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| options | GetUserBadgesOptions | Non |  |

## Réponse

Retourne : [`Option[APIGetUserBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badges_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetUserBadgesOptions()
let (badgesOpt, httpResp) = client.getUserBadges(tenantId = "my-tenant-123", options = opts)
if badgesOpt.isSome:
  let badges = badgesOpt.get()
[inline-code-end]

---