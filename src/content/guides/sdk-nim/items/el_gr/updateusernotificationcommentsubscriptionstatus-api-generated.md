## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| notificationId | string | Όχι |  |
| optedInOrOut | string | Όχι |  |
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationCommentSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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