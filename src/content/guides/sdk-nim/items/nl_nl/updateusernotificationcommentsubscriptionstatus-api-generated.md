---
Schakel meldingen in of uit voor een specifieke reactie.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Nee |  |
| optedInOrOut | string | Nee |  |
| commentId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationCommentSubscriptionStatus(
  tenantId = "my-tenant-123",
  notificationId = "",
  optedInOrOut = "",
  commentId = "cmt-789",
  sso = ""
)

if response.isSome:
  let updateResp = response.get()
  echo "Subscription update response: ", updateResp
[inline-code-end]

---