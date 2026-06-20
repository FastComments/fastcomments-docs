## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetTenantUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users_response.nim)

## Primjer

[inline-code-attrs-start title = 'getTenantUsers Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)

if response.isSome:
  let tenantUsers = response.get()
  echo "Retrieved tenant users"
else:
  echo "No tenant users returned"
[inline-code-end]

---