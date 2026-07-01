## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| notificationId | string | Ne |  |
| newStatus | string | Ne |  |
| sso | string = "" | Ne |  |

## Odgovor

Vrne: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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