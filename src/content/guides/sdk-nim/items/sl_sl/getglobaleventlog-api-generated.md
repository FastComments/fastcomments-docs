req
tenantId
urlId
userIdWS

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| userIdWS | string | Ne |  |
| startTime | int64 | Ne |  |
| endTime | int64 | Ne |  |

## Odgovor

Vrne: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getGlobalEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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