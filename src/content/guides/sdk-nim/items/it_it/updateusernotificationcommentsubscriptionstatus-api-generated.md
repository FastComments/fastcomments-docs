Abilita o disabilita le notifiche per un commento specifico.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | No |  |
| optedInOrOut | string | No |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## Risposta

Restituisce: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Esempio

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---