## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Oui |  |
| fromCommentId | string | Non |  |
| viewed | bool | Non |  |

## Réponse

Renvoie: [`Option[GetNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(
  tenantId = "acme-corp-tenant-12",
  userId = "user-84",
  urlId = "news/2026/market-update",
  fromCommentId = "cmt-20251234",
  viewed = false
)

if response.isSome:
  let notificationData = response.get()
  echo "Received notification data"
else:
  echo "No notification data"
[inline-code-end]

---