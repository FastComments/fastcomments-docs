## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Non |  |

## Réponse

Retourne : [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de mise à jour du badge d\'utilisateur'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateUserBadgeParams()
let (maybeResp, httpResp) = client.updateUserBadge(tenantId = "my-tenant-123", id = "user-456", updateUserBadgeParams = params)
if maybeResp.isSome:
  let success = maybeResp.get()
[inline-code-end]