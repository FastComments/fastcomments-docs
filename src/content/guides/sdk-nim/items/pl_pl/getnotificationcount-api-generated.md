## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| options | GetNotificationCountOptions | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (notifOpt, httpResp) = client.getNotificationCount(tenantId = "my-tenant-123", options = GetNotificationCountOptions())
if notifOpt.isSome:
  let notif = notifOpt.get()
  echo notif
[inline-code-end]