req
tenantId
urlId
userIdWS

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | No |  |
| startTime | int64 | No |  |
| endTime | int64 | No |  |

## Réponse

Renvoie : [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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