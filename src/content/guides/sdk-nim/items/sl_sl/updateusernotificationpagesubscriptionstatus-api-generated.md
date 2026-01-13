## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| url | string | Ne |  |
| pageTitle | string | Ne |  |
| subscribedOrUnsubscribed | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Primer

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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