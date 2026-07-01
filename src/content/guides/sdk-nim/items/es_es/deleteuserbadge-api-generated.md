## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if apiResponse.isSome:
  let success = apiResponse.get()
[inline-code-end]