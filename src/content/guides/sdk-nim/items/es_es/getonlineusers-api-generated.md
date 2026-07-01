Visitantes actualmente en línea de una página: personas cuya sesión websocket está suscrita a la página en este momento. Devuelve anonCount + totalCount (suscriptores de toda la sala, incluidos los espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| options | GetOnlineUsersOptions | No |  |

## Respuesta

Devuelve: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]