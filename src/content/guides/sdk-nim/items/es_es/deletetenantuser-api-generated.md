## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteTenantUserOptions | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteTenantUserOptions(),
)
if response.isSome:
  let empty = response.get()
  echo "User successfully deleted"
[inline-code-end]