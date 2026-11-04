페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새로운 루트 댓글에 대해 알림이 생성되며, 또한

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| url | string | 아니요 |  |
| pageTitle | string | 아니요 |  |
| subscribedOrUnsubscribed | string | 아니요 |  |
| sso | string = "" | 아니요 |  |

## 응답

반환값: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-456",
  url = "https://example.com/news/article-456",
  pageTitle = "Breaking News: Something Happened",
  subscribedOrUnsubscribed = "subscribed",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  # resp로 추가 처리
[inline-code-end]