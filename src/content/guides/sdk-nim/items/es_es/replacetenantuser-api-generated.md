## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| replaceTenantUserBody | ReplaceTenantUserBody | No |  |
| updateComments | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de replaceTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = ReplaceTenantUserBody(
  displayName = "Jane Doe",
  email = "jane.doe@example.com",
  externalId = "jdoe-789",
  admin = false,
  enabled = true,
  tags = @["editor", "subscriber"]
)

let (response, httpResponse) = client.replaceTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  replaceTenantUserBody = body,
  updateComments = "true"
)

if response.isSome:
  let apiEmpty = response.get()
  echo "ReplaceTenantUser succeeded, http status:", httpResponse.status
[inline-code-end]

---