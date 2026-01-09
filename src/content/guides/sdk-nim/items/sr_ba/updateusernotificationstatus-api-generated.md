## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| notificationId | string | Ne |  |
| newStatus | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  newStatus = "read",
  sso = "sso-abc-789"
)
if response.isSome:
  let updateResp = response.get()
  discard updateResp
[inline-code-end]

---