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

[inline-code-attrs-start title = 'getGlobalEventLog Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGlobalEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/2026/01/09/new-tech",
  userIdWS = "user-9876",
  startTime = 0,
  endTime = 0
)

if response.isSome:
  let eventLog = response.get()
  echo "Received global event log for my-tenant-123"
else:
  echo "No event log returned"
[inline-code-end]
