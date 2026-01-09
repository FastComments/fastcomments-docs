## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 否 |  |
| startTime | int64 | 否 |  |
| endTime | int64 | 否 |  |

## 回應

回傳: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## 範例

[inline-code-attrs-start title = 'getGlobalEventLog 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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