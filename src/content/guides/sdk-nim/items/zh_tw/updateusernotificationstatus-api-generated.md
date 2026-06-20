## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 否 |  |
| newStatus | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationStatus 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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