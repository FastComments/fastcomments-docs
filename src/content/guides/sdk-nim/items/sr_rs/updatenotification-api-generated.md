## Parametri

| Ime | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateNotificationBody | UpdateNotificationBody | Не |  |
| userId | string | Не |  |

## Одговор

Враћа: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Primer za updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-456",
  updateNotificationBody = UpdateNotificationBody(
    enabled = true,
    channels = @["email", "push"],
    frequency = "immediate"
  ),
  userId = "user-789"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---