## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | GetNotificationsOptions | Nee |  |

## Response

Retourneert: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Example

[inline-code-attrs-start title = 'getNotifications Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotifications(tenantId = "my-tenant-123", options = GetNotificationsOptions())
if notifOpt.isSome:
  let notifications = notifOpt.get()
[inline-code-end]