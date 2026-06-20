## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| notificationId | string | Ne |  |
| newStatus | string | Ne |  |
| sso | string | Ne |  |

## Odziv

Vrača: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  newStatus = "read",
  sso = "sso-token-abc123"
)
if response.isSome:
  let updated = response.get()
  echo "Notification status updated successfully"
else:
  echo "No update response received"
[inline-code-end]

---