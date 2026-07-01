req
tenantId
urlId
userIdWS

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| userIdWS | string | No |  |
| startTime | int64 | No |  |
| endTime | int64 | No |  |

## Respuesta

Devuelve: [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'getGlobalEventLog Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---