req
tenantId
urlId
userIdWS

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 否 |  |
| startTime | int64 | 否 |  |
| endTime | int64 | 否 |  |

## 响应

返回：[`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## 示例

[inline-code-attrs-start title = 'getEventLog 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (eventLogOpt, httpResp) = client.getEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userIdWS = "",
  startTime = 0'i64,
  endTime = 0'i64,
)

if eventLogOpt.isSome:
  let eventLog = eventLogOpt.get()
  echo eventLog
[inline-code-end]

---