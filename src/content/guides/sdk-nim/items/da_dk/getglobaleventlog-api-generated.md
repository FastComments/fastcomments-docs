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

[inline-code-attrs-start title = 'getGlobalEventLog Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---