## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| options | GetNotificationCountOptions | Nee |  |

## Respons

Retourneert: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getNotificationCount voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotificationCount(tenantId = "my-tenant-123", options = GetNotificationCountOptions())
if notifOpt.isSome:
  let notif = notifOpt.get()
  echo notif
[inline-code-end]