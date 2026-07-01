## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------|
| tenantId | string | Da |  |
| options | GetNotificationCountOptions | Ne |  |

## Odgovor

Vrne: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Primer

[inline-code-attrs-start title = 'getNotificationCount Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotificationCount(tenantId = "my-tenant-123", options = GetNotificationCountOptions())
if notifOpt.isSome:
  let notif = notifOpt.get()
  echo notif
[inline-code-end]