## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Nein |  |
| optedInOrOut | string | Nein |  |
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  optedInOrOut = "opted_in",
  commentId = "cmt-789",
  sso = "sso-token-abc"
)
if response.isSome:
  let updatedStatus = response.get()
  discard updatedStatus
else:
  discard httpResponse
[inline-code-end]

---