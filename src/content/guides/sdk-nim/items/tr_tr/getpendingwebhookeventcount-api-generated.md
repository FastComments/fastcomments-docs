## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| options | GetPendingWebhookEventCountOptions | Hayır |  |

## Yanıt

Döndürür: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEventCount Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pendingCountOpt, httpResponse) = client.getPendingWebhookEventCount(tenantId = "my-tenant-123", options = GetPendingWebhookEventCountOptions())
if pendingCountOpt.isSome:
  let pendingCount = pendingCountOpt.get()
  echo pendingCount
[inline-code-end]