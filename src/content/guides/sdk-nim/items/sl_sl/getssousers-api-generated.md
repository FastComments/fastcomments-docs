---
## Parametri

| Ime | Type | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | int | Ne |  |

## Odgovor

Vrne: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## Primer

[inline-code-attrs-start title = 'getSSOUsers Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo ssoUsers
else:
  echo "No SSO users returned; HTTP response:", httpResponse
[inline-code-end]

---