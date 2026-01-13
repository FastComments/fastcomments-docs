## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| externalId | string | 아니요 |  |
| eventType | string | 아니요 |  |
| domain | string | 아니요 |  |
| attemptCountGT | float64 | 아니요 |  |
| skip | float64 | 아니요 |  |

## 응답

반환: [`Option[GetPendingWebhookEvents_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events200response.nim)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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