페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새 루트 댓글에 대해 알림이 생성되며, 또한

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| url | string | 아니오 |  |
| pageTitle | string | 아니오 |  |
| subscribedOrUnsubscribed | string | 아니오 |  |
| sso | string | 아니오 |  |

## Response

반환: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/economy/market-rally-2026-06-19",
  url = "",
  pageTitle = "",
  subscribedOrUnsubscribed = "",
  sso = ""
)

if response.isSome:
  let updateResp = response.get()
  echo "Subscription update received: ", updateResp
else:
  echo "No subscription update returned."
[inline-code-end]