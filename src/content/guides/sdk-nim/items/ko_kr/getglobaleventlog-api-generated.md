req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 아니오 |  |
| startTime | int64 | 아니오 |  |
| endTime | int64 | 아니오 |  |

## 응답

반환: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## 예제

[inline-code-attrs-start title = 'getGlobalEventLog 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (eventLogOpt, httpResp) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userIdWS = "user-456",
  startTime = 1700000000'i64,
  endTime = 1700003600'i64,
)

if eventLogOpt.isSome:
  let eventLog = eventLogOpt.get()
  echo eventLog
[inline-code-end]