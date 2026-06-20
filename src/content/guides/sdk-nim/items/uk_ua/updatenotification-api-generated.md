## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateNotificationBody | UpdateNotificationBody | Ні |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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