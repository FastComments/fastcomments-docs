## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| urlId | string | Sì |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |
| skip | float64 | No |  |

## Risposta

Restituisce: [`Option[GetNotificationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(tenantId = "my-tenant-123", userId = "user-456", urlId = "news/article-title", fromCommentId = "cmt-789", viewed = false, skip = 0.0)
if response.isSome:
  let notifications = response.get()
  echo notifications
[inline-code-end]

---