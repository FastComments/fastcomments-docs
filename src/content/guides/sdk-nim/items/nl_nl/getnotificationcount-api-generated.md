## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| urlId | string | Ja |  |
| fromCommentId | string | Nee |  |
| viewed | bool | Nee |  |

## Response

Retourneert: [`Option[GetNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_notification_count_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getNotificationCount Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getNotificationCount(tenantId = "my-tenant-123", userId = "user-987", urlId = "news/2026/06/election-results", fromCommentId = "", viewed = false)
if response.isSome:
  let notifyData = response.get()
  echo notifyData
[inline-code-end]