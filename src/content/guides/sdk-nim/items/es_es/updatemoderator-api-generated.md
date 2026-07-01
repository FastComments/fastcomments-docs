## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| updateModeratorBody | UpdateModeratorBody | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateModeratorBody(name = "John Doe", email = "john@example.com", isActive = true)
let (apiResult, httpResp) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = body)
if apiResult.isSome:
  let result = apiResult.get()
[inline-code-end]