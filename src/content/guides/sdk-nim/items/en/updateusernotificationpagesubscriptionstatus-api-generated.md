Enable or disable notifications for a page. When users are subscribed to a page, notifications are created
for new root comments, and also

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
  urlId = "news/article-title-456",
  url = "",
  pageTitle = "",
  subscribedOrUnsubscribed = "",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Subscription update returned:", $result
[inline-code-end]
