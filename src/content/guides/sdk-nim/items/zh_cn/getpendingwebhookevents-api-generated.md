## 参数

| 名称 | 类型 | 必填 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | float64 | 否 |  |
| skip | float64 | 否 |  |

## 响应

返回：[`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEvents 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0,
  skip = 0.0
)
if response.isSome:
  let pending = response.get()
  discard pending
[inline-code-end]

---