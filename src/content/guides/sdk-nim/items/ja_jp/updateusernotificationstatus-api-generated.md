## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| notificationId | string | いいえ |  |
| newStatus | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## 例

[inline-code-attrs-start title = 'updateUserNotificationStatus の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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