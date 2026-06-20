## 參數

| 名稱 | Type | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | float64 | 否 |  |

## 回應

回傳: [`Option[GetPendingWebhookEventCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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