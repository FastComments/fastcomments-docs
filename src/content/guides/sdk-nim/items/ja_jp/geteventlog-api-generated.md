---
req
tenantId
urlId
userIdWS

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userIdWS | string | いいえ |  |
| startTime | int64 | いいえ |  |
| endTime | int64 | いいえ |  |

## レスポンス

返却値: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## 例

[inline-code-attrs-start title = 'getEventLog の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-solar-panels",
  userIdWS = "user-456",
  startTime = 1688000000'i64,
  endTime = 1688086400'i64
)
if response.isSome:
  let eventLog = response.get()
  discard eventLog
[inline-code-end]

---