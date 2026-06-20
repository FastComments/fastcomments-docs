## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| externalId | string | Hayır |  |
| eventType | string | Hayır |  |
| domain | string | Hayır |  |
| attemptCountGT | float64 | Hayır |  |

## Yanıt

Döndürür: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEventCount Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-456abc",
  externalId = "ext-7890",
  eventType = "comment_created",
  domain = "news.example.com",
  attemptCountGT = 2.0
)

if response.isSome:
  let pending = response.get()
  echo pending
else:
  echo "No pending webhook event count returned; HTTP status: ", httpResponse.status
[inline-code-end]

---