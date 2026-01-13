## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | float64 | No |  |

## Respuesta

Devuelve: [`Option[GetTenantUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTenantUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let tenantUsers = response.get()
  echo "Fetched tenant users for my-tenant-123"
  discard tenantUsers
else:
  echo "No users returned"
  discard httpResponse
[inline-code-end]

---