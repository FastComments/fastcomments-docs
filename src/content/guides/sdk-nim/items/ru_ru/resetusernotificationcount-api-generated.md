## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример resetUserNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.resetUserNotificationCount(tenantId = "my-tenant-123", sso = "user-sso-token-456")
if response.isSome:
  let result = response.get()
  echo "ResetUserNotificationsResponse:", result
else:
  echo "Reset failed, HTTP response:", httpResponse
[inline-code-end]

---