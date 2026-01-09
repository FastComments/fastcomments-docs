## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| userIdWS | string | Ne |  |
| startTime | int64 | Ne |  |
| endTime | int64 | Ne |  |

## Odgovor

Vraća: [`Option[GetEventLog_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log200response.nim)

## Primjer

[inline-code-attrs-start title = 'getEventLog Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEventLog(
  tenantId = "my-tenant-123",
  urlId = "news/politics/election-2024",
  userIdWS = "",
  startTime = 0'i64,
  endTime = 0'i64
)
if response.isSome:
  let eventLog = response.get()
  echo "Received event log for ", "my-tenant-123"
else:
  echo "No event log returned. HTTP status: ", $httpResponse.status
[inline-code-end]

---