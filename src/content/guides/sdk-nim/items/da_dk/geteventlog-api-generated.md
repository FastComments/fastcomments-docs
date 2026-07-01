req
tenantId
urlId
userIdWS

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Nej |  |
| startTime | int64 | Nej |  |
| endTime | int64 | Nej |  |

## Svar

Returnerer: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getEventLog Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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