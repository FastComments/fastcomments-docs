## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 否 |  |
| startTime | int64 | 否 |  |
| endTime | int64 | 否 |  |

## 响应

返回: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## 示例

[inline-code-attrs-start title = 'getGlobalEventLog 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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