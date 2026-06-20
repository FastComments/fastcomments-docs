## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| urlId | string | Так |  |
| fromCommentId | string | Ні |  |
| viewed | bool | Ні |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(tenantId = "my-tenant-123", userId = "user-456", urlId = "news/article-title", fromCommentId = "cmt-789", viewed = false, skip = 0.0)
if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---