---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |

## Risposta

Restituisce: [`Option[GetTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-789")
if response.isSome:
  let tenantUser = response.get()
  echo "User fetched:", tenantUser
else:
  echo "No user found, HTTP status:", httpResponse.status
[inline-code-end]

---