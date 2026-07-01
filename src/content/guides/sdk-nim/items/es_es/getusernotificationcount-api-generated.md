## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| sso | string = "" | No |  |

## Respuesta

Devuelve: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]