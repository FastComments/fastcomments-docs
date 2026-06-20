Włącz lub wyłącz powiadomienia dla strony. Gdy użytkownicy subskrybują stronę, tworzone są powiadomienia
o nowych komentarzach głównych, oraz także

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| url | string | Nie |  |
| pageTitle | string | Nie |  |
| subscribedOrUnsubscribed | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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