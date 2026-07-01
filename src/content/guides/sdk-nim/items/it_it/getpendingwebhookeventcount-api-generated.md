## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | GetPendingWebhookEventCountOptions | No |  |

## Risposta

Restituisce: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Esempio

[inline-code-attrs-start title = 'getPendingWebhookEventCount Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pendingCountOpt, httpResponse) = client.getPendingWebhookEventCount(tenantId = "my-tenant-123", options = GetPendingWebhookEventCountOptions())
if pendingCountOpt.isSome:
  let pendingCount = pendingCountOpt.get()
  echo pendingCount
[inline-code-end]