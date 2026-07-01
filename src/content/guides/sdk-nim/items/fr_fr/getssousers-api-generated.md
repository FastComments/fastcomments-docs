---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| skip | int | Non |  |

## Réponse

Retourne : [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSSOUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if maybeResponse.isSome:
  let users = maybeResponse.get()
  echo users
else:
  echo "No SSO users found"
echo httpResponse.statusCode
[inline-code-end]

---