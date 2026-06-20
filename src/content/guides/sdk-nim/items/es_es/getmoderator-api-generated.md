## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-456")
if response.isSome:
  let moderator = response.get()
  echo moderator
else:
  echo "Moderator not found, HTTP status: ", $httpResponse.status
[inline-code-end]

---