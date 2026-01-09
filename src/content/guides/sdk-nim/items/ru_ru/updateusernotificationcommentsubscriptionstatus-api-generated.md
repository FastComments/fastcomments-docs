## Параметры

| Имя | Тип | Обязательное | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| notificationId | string | Нет |  |
| optedInOrOut | string | Нет |  |
| commentId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  optedInOrOut = "opted_in",
  commentId = "cmt-789",
  sso = "sso-token-abc"
)
if response.isSome:
  let updatedStatus = response.get()
  discard updatedStatus
else:
  discard httpResponse
[inline-code-end]

---