---
Benachrichtigungen für eine Seite aktivieren oder deaktivieren. Wenn Benutzer für eine Seite abonniert sind, werden Benachrichtigungen erstellt
für neue Root-Kommentare, und auch

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Nein |  |
| pageTitle | string | Nein |  |
| subscribedOrUnsubscribed | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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