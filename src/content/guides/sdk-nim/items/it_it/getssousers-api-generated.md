## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| skip | int | No |  |

## Risposta

Restituisce: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getSSOUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if response.isSome:
  let ssoUsers = response.get()
  echo ssoUsers
else:
  echo "No SSO users returned; HTTP response:", httpResponse
[inline-code-end]

---