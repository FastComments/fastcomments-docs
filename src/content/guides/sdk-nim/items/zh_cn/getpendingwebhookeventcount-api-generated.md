## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | float64 | 否 |  |

## 响应

返回: [`Option[GetPendingWebhookEventCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count200response.nim)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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