Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, yeni kök yorumlar için bildirimler oluşturulur ve ayrıca

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| url | string | Hayır |  |
| pageTitle | string | Hayır |  |
| subscribedOrUnsubscribed | string | Hayır |  |
| sso | string = "" | Hayır |  |

## Yanıt

Döndürür: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # resp ile ileri işlemler
[inline-code-end]