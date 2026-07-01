---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetNotificationsOptions | Нет |  |

## Ответ

Возвращает: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotifications(tenantId = "my-tenant-123", options = GetNotificationsOptions())
if notifOpt.isSome:
  let notifications = notifOpt.get()
[inline-code-end]

---