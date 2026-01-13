---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| urlId | string | Da |  |
| fromCommentId | string | Ne |  |
| viewed | bool | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetNotifications_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notifications200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getNotifications'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotifications(
  tenantId = "fastcomments-tenant-42",
  userId = "",
  urlId = "news/latest-tech-innovations",
  fromCommentId = "",
  viewed = false,
  skip = 0.0
)

if response.isSome:
  let notifications = response.get()
  echo "Received notifications: ", notifications
else:
  echo "No notifications, response: ", httpResponse
[inline-code-end]

---