## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| updateNotificationBody | UpdateNotificationBody | Нет |  |
| userId | string = "" | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'updateNotification Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateNotificationBody(message: "Your comment was approved", isRead: false)
let (optResp, httpResp) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-789",
  updateNotificationBody = body,
  userId = "user-42"
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]