특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| notificationId | string | 아니오 |  |
| optedInOrOut | string | 아니오 |  |
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[UpdateUserNotificationCommentSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_comment_subscription_status_response.nim)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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