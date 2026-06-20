---
Включване или изключване на известия за страница. Когато потребителите са абонирани за страница, се създават известия за нови коренни коментари, и също така

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| url | string | Не |  |
| pageTitle | string | Не |  |
| subscribedOrUnsubscribed | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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