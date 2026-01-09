## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Nein |  |
| newStatus | string | Nein |  |
| sso | string | Nein |  |

## Response

Rückgabe: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationStatus Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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