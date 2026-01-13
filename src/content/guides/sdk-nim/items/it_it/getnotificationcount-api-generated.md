## Parametri

| Nome | Type | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| urlId | string | Sì |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |

## Risposta

Restituisce: [`Option[GetNotificationCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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