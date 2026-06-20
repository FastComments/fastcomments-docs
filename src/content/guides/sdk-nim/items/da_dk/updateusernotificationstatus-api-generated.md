## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Nej |  |
| newStatus | string | Nej |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`Option[UpdateUserNotificationStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateUserNotificationStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  newStatus = "read",
  sso = "sso-token-abc123"
)
if response.isSome:
  let updated = response.get()
  echo "Notification status updated successfully"
else:
  echo "No update response received"
[inline-code-end]

---