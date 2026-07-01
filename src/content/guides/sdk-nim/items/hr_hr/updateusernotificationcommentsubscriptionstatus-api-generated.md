Omogućite ili onemogućite obavijesti za određeni komentar.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| notificationId | string | Ne |  |
| optedInOrOut | string | Ne |  |
| commentId | string | Da |  |
| sso | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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