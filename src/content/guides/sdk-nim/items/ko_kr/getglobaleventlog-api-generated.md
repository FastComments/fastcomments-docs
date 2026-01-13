## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userIdWS | string | 아니오 |  |
| startTime | int64 | 아니오 |  |
| endTime | int64 | 아니오 |  |

## 응답

반환: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## 예제

[inline-code-attrs-start title = 'getGlobalEventLog 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userIdWS = "",
  startTime = int64(0),
  endTime = int64(0)
)
if response.isSome:
  let eventLog = response.get()
  echo eventLog
else:
  echo "No event log returned"
[inline-code-end]

---