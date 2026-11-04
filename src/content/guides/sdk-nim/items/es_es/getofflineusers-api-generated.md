Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Úselo después de agotar /users/online para renderizar una sección "Members".
Paginación por cursor sobre commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| options | GetOfflineUsersOptions | No |  |

## Respuesta

Devuelve: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]