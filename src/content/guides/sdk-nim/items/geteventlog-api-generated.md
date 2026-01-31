## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | No |  |
| startTime | int64 | No |  |
| endTime | int64 | No |  |

## Response

Returns: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## Example

[inline-code-attrs-start title = 'getEventLog Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEventLog(tenantId = "my-tenant-123",
  urlId = "news/breaking-story",
  userIdWS = "",
  startTime = 0'i64,
  endTime = 0'i64)

if response.isSome:
  let eventLog = response.get()
  discard eventLog
[inline-code-end]
