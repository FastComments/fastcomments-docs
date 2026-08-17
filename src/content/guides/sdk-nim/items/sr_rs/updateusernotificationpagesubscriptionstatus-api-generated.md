Омогућите или онемогућите обавештења за страницу. Када су корисници претплаћени на страницу, обавештења се креирају за нове коренске коментаре, а такође

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| url | string | No |  |
| pageTitle | string | No |  |
| subscribedOrUnsubscribed | string | No |  |
| sso | string = "" | No |  |

## Одговор

Враћа: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Пример

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # даља обрада са resp
[inline-code-end]