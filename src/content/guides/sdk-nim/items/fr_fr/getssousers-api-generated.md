---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | int | Non |  |

## Réponse

Renvoie: [`Option[GetSSOUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSSOUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo "Fetched SSO users:"
  echo ssoUsers
else:
  echo "No SSO users returned, HTTP status: ", httpResponse.statusCode
[inline-code-end]

---