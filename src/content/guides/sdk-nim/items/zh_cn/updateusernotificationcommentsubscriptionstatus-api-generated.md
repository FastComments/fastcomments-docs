为特定评论启用或禁用通知。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| notificationId | string | 否 |  |
| optedInOrOut | string | 否 |  |
| commentId | string | 是 |  |
| sso | string = "" | 否 |  |

## 响应

返回: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## 示例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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