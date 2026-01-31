## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| url | string | No |  |
| pageTitle | string | No |  |
| subscribedOrUnsubscribed | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Example

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-01-09",
  url = "https://www.news-site.com/articles/2026/01/09/article-title",
  pageTitle = "Breaking News: Major Event",
  subscribedOrUnsubscribed = "subscribed",
  sso = "ssoToken_abc123"
)

if response.isSome:
  let result = response.get()
  echo "Subscription update succeeded:", result
else:
  echo "Subscription update failed, HTTP status:", httpResponse.status
[inline-code-end]
