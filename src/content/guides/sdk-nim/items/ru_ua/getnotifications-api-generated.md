## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| options | GetNotificationsOptions | Ні |  |

## Response

Повертає: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Example

[inline-code-attrs-start title = 'Приклад getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotifications(tenantId = "my-tenant-123", options = GetNotificationsOptions())
if notifOpt.isSome:
  let notifications = notifOpt.get()
[inline-code-end]