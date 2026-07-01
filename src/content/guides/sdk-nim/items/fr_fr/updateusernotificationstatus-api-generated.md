## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| notificationId | string | Non |  |
| newStatus | string | Non |  |
| sso | string = "" | Non |  |

## Réponse

Renvoie : [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Exemple

[inline-code-attrs-start title = 'updateUserNotificationStatus Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResponse) = client.updateUserNotificationStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  newStatus = "read",
  sso = ""
)
if respOpt.isSome:
  let status = respOpt.get()
[inline-code-end]

---