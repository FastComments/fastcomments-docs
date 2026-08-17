## Parameters

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetUserNotificationsOptions | No |  |

## Response

Враћа: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Example

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getUserNotifications(tenantId = "my-tenant-123", options = GetUserNotificationsOptions())
if maybeResponse.isSome:
  let notifications = maybeResponse.get()
[inline-code-end]

---