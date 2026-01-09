## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 否 |  |
| optedInOrOut | string | 否 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳： [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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