## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "tenant-789")
if response.isSome:
  let tenant = response.get()
  discard tenant
else:
  discard httpResponse
[inline-code-end]

---