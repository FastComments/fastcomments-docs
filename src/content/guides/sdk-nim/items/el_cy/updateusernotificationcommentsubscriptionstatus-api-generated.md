Ενεργοποιήστε ή απενεργοποιήστε τις ειδοποιήσεις για ένα συγκεκριμένο σχόλιο.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| notificationId | string | Όχι |  |
| optedInOrOut | string | Όχι |  |
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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