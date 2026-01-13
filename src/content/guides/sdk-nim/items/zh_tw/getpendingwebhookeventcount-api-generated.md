## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| externalId | string | No |  |
| eventType | string | No |  |
| domain | string | No |  |
| attemptCountGT | float64 | No |  |

## 回應

回傳: [`Option[GetPendingWebhookEventCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count200response.nim)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-4567",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0
)
if response.isSome:
  let pending = response.get()
  echo "Received pending webhook event count response: ", $pending
else:
  echo "No pending webhook event count returned, HTTP response: ", $httpResponse
[inline-code-end]

---