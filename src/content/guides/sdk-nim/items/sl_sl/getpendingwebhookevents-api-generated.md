## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | float64 | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vrne: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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