## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| urlId | string | Да |  |
| fromCommentId | string | Не |  |
| viewed | bool | Не |  |

## Одговор

Враћа: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Пример

[inline-code-attrs-start title = 'getNotificationCount Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(tenantId = "my-tenant-123", userId = "user-987", urlId = "news/2026/06/election-results", fromCommentId = "", viewed = false)
if response.isSome:
  let notifyData = response.get()
  echo notifyData
[inline-code-end]

---