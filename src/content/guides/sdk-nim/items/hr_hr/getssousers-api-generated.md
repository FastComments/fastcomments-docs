## Parametri

| Ime | Tip | Potrebno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| skip | int | Ne |  |

## Odgovor

Vraća: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## Primjer

[inline-code-attrs-start title = 'getSSOUsers Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if maybeResponse.isSome:
  let users = maybeResponse.get()
  echo users
else:
  echo "No SSO users found"
echo httpResponse.statusCode
[inline-code-end]