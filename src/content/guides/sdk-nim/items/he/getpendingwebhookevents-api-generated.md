## פרמטרים

| שם | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| externalId | string | לא |  |
| eventType | string | לא |  |
| domain | string | לא |  |
| attemptCountGT | float64 | לא |  |
| skip | float64 | לא |  |

## תגובה

מחזיר: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0,
  skip = 0.0
)
if response.isSome:
  let pending = response.get()
  discard pending
[inline-code-end]

---