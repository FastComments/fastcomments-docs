## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| externalId | string | No |  |
| eventType | string | No |  |
| domain | string | No |  |
| attemptCountGT | float64 | No |  |
| skip | float64 | No |  |

## Yanıt

Döndürür: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEvents Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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