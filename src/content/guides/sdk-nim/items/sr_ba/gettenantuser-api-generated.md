---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |

## Одговор

Vraća: [`Option[GetTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-789")
if response.isSome:
  let tenantUser = response.get()
  echo "User fetched:", tenantUser
else:
  echo "No user found, HTTP status:", httpResponse.status
[inline-code-end]

---