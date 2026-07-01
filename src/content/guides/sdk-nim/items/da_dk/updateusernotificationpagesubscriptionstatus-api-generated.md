---
Aktiver eller deaktiver meddelelser for en side. Når brugere er abonneret på en side, oprettes meddelelser for nye rodkommentarer, og også

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Nej |  |
| pageTitle | string | Nej |  |
| subscribedOrUnsubscribed | string | Nej |  |
| sso | string = "" | Nej |  |

## Svar

Returnerer: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Eksempel

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  # yderligere behandling med resp
[inline-code-end]

---