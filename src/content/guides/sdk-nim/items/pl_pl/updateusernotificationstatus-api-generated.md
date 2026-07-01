## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| notificationId | string | Nie |  |
| newStatus | string | Nie |  |
| sso | string = "" | Nie |  |

## Odpowiedź

Zwraca: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Przykład

[inline-code-attrs-start title = 'updateUserNotificationStatus Przykład'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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