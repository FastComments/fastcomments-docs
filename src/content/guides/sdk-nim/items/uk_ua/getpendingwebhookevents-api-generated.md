## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| externalId | string | Ні |  |
| eventType | string | Ні |  |
| domain | string | Ні |  |
| attemptCountGT | float64 | Ні |  |
| skip | float64 | Ні |  |

## Response

Повертає: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Example

[inline-code-attrs-start title = 'Приклад getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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