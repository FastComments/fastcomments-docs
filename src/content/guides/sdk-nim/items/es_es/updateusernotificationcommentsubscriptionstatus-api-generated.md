---
Habilitar o deshabilitar las notificaciones para un comentario específico.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| notificationId | string | No |  |
| optedInOrOut | string | No |  |
| commentId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "",
  optedInOrOut = "",
  commentId = "cmt-789",
  sso = ""
)

if response.isSome:
  let updateResp = response.get()
  echo "Subscription update response: ", updateResp
[inline-code-end]

---