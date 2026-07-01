---
req
tenantId
urlId
userIdWS

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| userIdWS | string | Non |  |
| startTime | int64 | Non |  |
| endTime | int64 | Non |  |

## Réponse

Renvoie : [`Option[GetEventLogResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_event_log_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getGlobalEventLog'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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