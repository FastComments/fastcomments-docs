## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetUserNotificationsOptions | Ne |  |

## Response

Vrne: [`Option[GetMyNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_my_notifications_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getUserNotifications(tenantId = "my-tenant-123", options = GetUserNotificationsOptions())
if maybeResponse.isSome:
  let notifications = maybeResponse.get()
[inline-code-end]