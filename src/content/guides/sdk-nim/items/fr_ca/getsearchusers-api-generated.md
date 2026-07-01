---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| options | GetSearchUsersOptions | Non |  |

## Réponse

Retourne : [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## Exemple

[inline-code-attrs-start title = 'getSearchUsers Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchRes, httpRes) = client.getSearchUsers(tenantId = "my-tenant-123", options = default(GetSearchUsersOptions))
if searchRes.isSome:
  let data = searchRes.get()
  echo data
[inline-code-end]

---