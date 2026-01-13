## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| externalId | string | いいえ |  |
| eventType | string | いいえ |  |
| domain | string | いいえ |  |
| attemptCountGT | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetPendingWebhookEventCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count200response.nim)

## 例

[inline-code-attrs-start title = 'getPendingWebhookEventCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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