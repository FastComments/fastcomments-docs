Schakel meldingen voor een pagina in of uit. Wanneer gebruikers zich op een pagina abonneren, worden meldingen aangemaakt voor nieuwe root-reacties, en ook

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Nee |  |
| pageTitle | string | Nee |  |
| subscribedOrUnsubscribed | string | Nee |  |
| sso | string | Nee |  |

## Antwoord

Retourneert: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---