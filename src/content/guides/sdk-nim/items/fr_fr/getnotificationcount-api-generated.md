## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Oui |  |
| fromCommentId | string | Non |  |
| viewed | bool | Non |  |

## Réponse

Retourne: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotificationCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(tenantId = "my-tenant-123", userId = "user-987", urlId = "news/2026/06/election-results", fromCommentId = "", viewed = false)
if response.isSome:
  let notifyData = response.get()
  echo notifyData
[inline-code-end]

---