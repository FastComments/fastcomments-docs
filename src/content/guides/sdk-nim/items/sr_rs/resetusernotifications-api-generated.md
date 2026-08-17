## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | ResetUserNotificationsOptions | Не |  |

## Одговор

Враћа: [`Option[ResetUserNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_reset_user_notifications_response.nim)

## Пример

[inline-code-attrs-start title = 'resetUserNotifications Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.resetUserNotifications(
  tenantId = "my-tenant-123",
  options = ResetUserNotificationsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]