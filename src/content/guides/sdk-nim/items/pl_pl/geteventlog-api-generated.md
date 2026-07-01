req
tenantId
urlId
userIdWS

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| userIdWS | string | Nie |  |
| startTime | int64 | Nie |  |
| endTime | int64 | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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