req
tenantId
urlId
userIdWS

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 아니요 |  |
| startTime | int64 | 아니요 |  |
| endTime | int64 | 아니요 |  |

## 응답

반환: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## 예제

[inline-code-attrs-start title = 'getGlobalEventLog 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-2026-06-19",
  userIdWS = "user-987",
  startTime = int64(1622505600),
  endTime = int64(1625097600)
)
if response.isSome:
  let eventLog = response.get()
  echo eventLog, httpResponse.statusCode
[inline-code-end]