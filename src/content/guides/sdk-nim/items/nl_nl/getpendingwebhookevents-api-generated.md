## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | GetPendingWebhookEventsOptions | Nee |  |

## Respons

Retourneert: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEvents Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  options = GetPendingWebhookEventsOptions()
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]