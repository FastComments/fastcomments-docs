## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | float64 | 否 |  |
| skip | float64 | 否 |  |

## 回應

回傳: [`Option[GetPendingWebhookEvents_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events200response.nim)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEvents 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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