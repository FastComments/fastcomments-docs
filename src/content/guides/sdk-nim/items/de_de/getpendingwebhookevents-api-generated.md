## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| options | GetPendingWebhookEventsOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getPendingWebhookEvents Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  options = GetPendingWebhookEventsOptions()
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]