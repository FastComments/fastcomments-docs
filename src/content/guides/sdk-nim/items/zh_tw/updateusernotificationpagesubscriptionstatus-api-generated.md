## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| url | string | 否 |  |
| pageTitle | string | 否 |  |
| subscribedOrUnsubscribed | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-2025-11-22",
  url = "https://example.com/news/article-2025-11-22",
  pageTitle = "Breaking News: Market Update",
  subscribedOrUnsubscribed = "subscribed",
  sso = "sso-token-abc123"
)

if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---