## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| externalId | string | Nee |  |
| eventType | string | Nee |  |
| domain | string | Nee |  |
| attemptCountGT | float64 | Nee |  |
| skip | float64 | Nee |  |

## Antwoord

Geeft terug: [`Option[GetPendingWebhookEvents_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEvents Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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