## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | float64 | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vrne: [`Option[GetPendingWebhookEvents_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events200response.nim)

## Primer

[inline-code-attrs-start title = 'getPendingWebhookEvents Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0,
  skip = 0.0
)
if response.isSome:
  let pending = response.get()
  discard pending
  echo "Received pending webhook events"
else:
  echo "No pending webhook events"
[inline-code-end]

---