## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| updateNotificationBody | UpdateNotificationBody | Non |  |
| userId | string = "" | Non |  |

## Réponse

Renvoie : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateNotificationBody(message: "Your comment was approved", isRead: false)
let (optResp, httpResp) = client.updateNotification(
  tenantId = "my-tenant-123",
  id = "notif-789",
  updateNotificationBody = body,
  userId = "user-42"
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]

---