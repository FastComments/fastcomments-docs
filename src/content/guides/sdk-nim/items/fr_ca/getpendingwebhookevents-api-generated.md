## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| externalId | string | Non |  |
| eventType | string | Non |  |
| domain | string | Non |  |
| attemptCountGT | float64 | Non |  |
| skip | float64 | Non |  |

## Réponse

Retourne : [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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