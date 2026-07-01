req
tenantId
urlId
userIdWS

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Nein |  |
| startTime | int64 | Nein |  |
| endTime | int64 | Nein |  |

## Antwort

Rückgabe: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getGlobalEventLog Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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