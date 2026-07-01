## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateNotificationBody | UpdateNotificationBody | Ne |  |
| userId | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer updateNotification'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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