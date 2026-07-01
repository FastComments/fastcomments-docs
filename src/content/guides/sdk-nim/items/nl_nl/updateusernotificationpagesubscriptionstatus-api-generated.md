Enable of uitschakelen van meldingen voor een pagina. Wanneer gebruikers geabonneerd zijn op een pagina, worden meldingen aangemaakt voor nieuwe hoofdreacties, en ook

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Nee |  |
| pageTitle | string | Nee |  |
| subscribedOrUnsubscribed | string | Nee |  |
| sso | string = "" | Nee |  |

## Respons

Retourneert: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # verdere verwerking met resp
[inline-code-end]