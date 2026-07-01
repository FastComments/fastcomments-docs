## Parameter

| Name   | Typ    | Erforderlich | Beschreibung |
|--------|--------|--------------|--------------|
| tenantId | string | Ja |  |
| options  | GetNotificationsOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getNotifications Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotifications(tenantId = "my-tenant-123", options = GetNotificationsOptions())
if notifOpt.isSome:
  let notifications = notifOpt.get()
[inline-code-end]