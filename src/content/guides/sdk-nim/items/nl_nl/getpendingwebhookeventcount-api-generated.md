## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | GetPendingWebhookEventCountOptions | Nee |  |

## Response

Retourneert: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pendingCountOpt, httpResponse) = client.getPendingWebhookEventCount(tenantId = "my-tenant-123", options = GetPendingWebhookEventCountOptions())
if pendingCountOpt.isSome:
  let pendingCount = pendingCountOpt.get()
  echo pendingCount
[inline-code-end]