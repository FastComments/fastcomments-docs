## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| email | string | Non |  |

## Réponse

Renvoie : [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getSSOUserByEmail'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "john.doe@example.com")
if optResp.isSome:
  let user = optResp.get()
  discard user
[inline-code-end]

---