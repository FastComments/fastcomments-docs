## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let badgeId = "badge-456"

let (response, httpResponse) = client.deleteUserBadge(tenantId = tenantId, id = badgeId)

if response.isSome:
  let success = response.get()
  echo "Badge deleted successfully for tenant: ", tenantId, " id: ", badgeId
else:
  echo "Failed to delete badge. HTTP status: ", $httpResponse.status
[inline-code-end]

---