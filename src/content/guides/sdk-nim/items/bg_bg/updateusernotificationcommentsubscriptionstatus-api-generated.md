Активирайте или деактивирайте известия за конкретен коментар.

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| notificationId | string | Не |  |
| optedInOrOut | string | Не |  |
| commentId | string | Да |  |
| sso | string = "" | Не |  |

## Отговор

Връща: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  optedInOrOut = "opted-in",
  commentId = "comment-789",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]