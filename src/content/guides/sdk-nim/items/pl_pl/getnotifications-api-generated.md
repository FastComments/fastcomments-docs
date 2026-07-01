## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetNotificationsOptions | No |  |

## Odpowiedź

Zwraca: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotifications(tenantId = "my-tenant-123", options = GetNotificationsOptions())
if notifOpt.isSome:
  let notifications = notifOpt.get()
[inline-code-end]