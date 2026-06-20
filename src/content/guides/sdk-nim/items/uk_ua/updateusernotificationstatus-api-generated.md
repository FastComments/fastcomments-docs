## Параметри

| Назва | Type | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| notificationId | string | Ні |  |
| newStatus | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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