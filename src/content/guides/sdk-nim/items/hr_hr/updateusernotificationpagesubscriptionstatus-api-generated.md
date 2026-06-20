Omogućite ili onemogućite obavijesti za stranicu. Kada su korisnici pretplaćeni na stranicu, obavijesti se stvaraju za nove glavne komentare, i također

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| url | string | Ne |  |
| pageTitle | string | Ne |  |
| subscribedOrUnsubscribed | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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