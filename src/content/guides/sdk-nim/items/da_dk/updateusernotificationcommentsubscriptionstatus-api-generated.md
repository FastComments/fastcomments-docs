Aktivér eller deaktiver notifikationer for en specifik kommentar.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Nej |  |
| optedInOrOut | string | Nej |  |
| commentId | string | Ja |  |
| sso | string = "" | Nej |  |

## Svar

Returnerer: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Eksempel

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "notif-456",
  optedInOrOut = "opted-in",
  commentId = "comment-789",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]