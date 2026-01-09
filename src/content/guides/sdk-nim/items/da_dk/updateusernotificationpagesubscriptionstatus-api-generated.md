## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Nej |  |
| pageTitle | string | Nej |  |
| subscribedOrUnsubscribed | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[UpdateUserNotificationStatus_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_status200response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateUserNotificationPageSubscriptionStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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